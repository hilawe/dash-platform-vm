//! Module bindings, step 1: the Querier backend over Dash-native state in GroveDB.
//!
//! Implements the real `cosmwasm_vm::Querier` trait over Dash-native token state held in GroveDB,
//! answering a standard bank balance query (a Dash token as a denom) and a custom Dash query (token
//! supply). Each answer carries gas derived from the REAL GroveDB read cost, and a corrupt stored
//! balance is surfaced as an error rather than silently read as zero.

use cosmwasm_host::{bank, cost_to_gas, gv, is_not_found};
use cosmwasm_std::{
    from_json, to_json_binary, BalanceResponse, BankQuery, Binary, Coin, ContractResult, CustomQuery,
    QueryRequest, SystemResult,
};
use cosmwasm_vm::{BackendResult, GasInfo, Querier};
use grovedb::{Element, GroveDb};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const ROOT: &[&[u8]] = &[];
const BANK: &[u8] = b"bank";
const SUPPLY: &[u8] = b"supply";

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum DashQuery {
    TokenSupply { denom: String },
}
impl CustomQuery for DashQuery {}

#[derive(Serialize, Deserialize)]
struct SupplyResponse {
    amount: Coin,
}

struct GroveQuerier {
    db: Arc<GroveDb>,
}

impl GroveQuerier {
    /// Read an amount from a subtree, returning the value and the read's real gas. A missing key is
    /// zero; a present-but-corrupt value (not UTF-8, not an integer) is an error, NOT zero.
    fn read_amount(&self, subtree: &[u8], key: &[u8]) -> (Result<u128, String>, u64) {
        let cc = self.db.get([subtree].as_ref(), key, None, gv());
        let gas = cost_to_gas(&cc.cost);
        // Reuse the shared bank codec so the balance/amount decoding is defined in exactly one place.
        let result = match cc.value {
            Ok(Element::Item(bytes, _)) => bank::decode_balance(&bytes),
            Ok(other) => Err(format!("unexpected element type: {other:?}")),
            Err(ref e) if is_not_found(e) => Ok(0),
            Err(e) => Err(format!("grovedb get: {e}")),
        };
        (result, gas)
    }
}

impl Querier for GroveQuerier {
    fn query_raw(
        &self,
        request: &[u8],
        _gas_limit: u64,
    ) -> BackendResult<SystemResult<ContractResult<Binary>>> {
        // The caller-controlled request is parsed before any read, so charge for that parsing on EVERY
        // return path (malformed and unsupported included), never free gas for unbounded input.
        let req_gas = cosmwasm_host::request_gas(request.len());
        let req: QueryRequest<DashQuery> = match from_json(request) {
            Ok(r) => r,
            Err(e) => {
                return (
                    Ok(SystemResult::Ok(ContractResult::Err(format!("invalid query request: {e}")))),
                    GasInfo::with_cost(req_gas),
                )
            }
        };
        let (subtree, key, denom): (&[u8], Vec<u8>, String) = match req {
            QueryRequest::Bank(BankQuery::Balance { address, denom }) => {
                (BANK, format!("{denom}:{address}").into_bytes(), denom)
            }
            QueryRequest::Custom(DashQuery::TokenSupply { denom }) => {
                (SUPPLY, denom.as_bytes().to_vec(), denom)
            }
            _ => {
                return (
                    Ok(SystemResult::Ok(ContractResult::Err("unsupported query".to_string()))),
                    GasInfo::with_cost(req_gas),
                )
            }
        };
        let (amount, gas) = self.read_amount(subtree, &key);
        let gas = gas.saturating_add(req_gas);
        let amount = match amount {
            Ok(a) => a,
            // Corruption is surfaced to the contract, not hidden as a zero balance.
            Err(e) => {
                return (
                    Ok(SystemResult::Ok(ContractResult::Err(e))),
                    GasInfo::with_cost(gas),
                )
            }
        };
        let binary = if subtree == BANK {
            to_json_binary(&BalanceResponse { amount: Coin::new(amount, denom) }).unwrap()
        } else {
            to_json_binary(&SupplyResponse { amount: Coin::new(amount, denom) }).unwrap()
        };
        (Ok(SystemResult::Ok(ContractResult::Ok(binary))), GasInfo::with_cost(gas))
    }
}

fn seed(db: &GroveDb) {
    for tree in [BANK, SUPPLY] {
        db.insert(ROOT, tree, Element::empty_tree(), None, None, gv())
            .unwrap()
            .expect("subtree");
    }
    let set = |subtree: &[u8], key: &str, value: &[u8]| {
        db.insert([subtree].as_ref(), key.as_bytes(), Element::new_item(value.to_vec()), None, None, gv())
            .unwrap()
            .expect("seed");
    };
    set(BANK, "udash:alice", b"1000");
    set(BANK, "udash:bob", b"500");
    set(SUPPLY, "udash", b"1500");
    set(BANK, "udash:mallory", b"not-a-number"); // a corrupt balance, to exercise error surfacing
}

fn ask<T: serde::de::DeserializeOwned>(q: &GroveQuerier, request: &QueryRequest<DashQuery>) -> Result<(T, u64), String> {
    let bytes = cosmwasm_std::to_json_vec(request).unwrap();
    let (res, gas) = q.query_raw(&bytes, u64::MAX);
    let sys = res.expect("backend ok");
    let contract = match sys {
        SystemResult::Ok(c) => c,
        SystemResult::Err(e) => return Err(format!("system error: {e:?}")),
    };
    match contract {
        ContractResult::Ok(b) => Ok((from_json::<T>(&b).unwrap(), gas.cost)),
        ContractResult::Err(e) => Err(e),
    }
}

fn balance_query(address: &str) -> QueryRequest<DashQuery> {
    QueryRequest::Bank(BankQuery::Balance { address: address.to_string(), denom: "udash".to_string() })
}

fn main() {
    println!("# Module bindings, step 1: the cosmwasm_vm::Querier trait over Dash-native state in GroveDB.");
    println!("# A Dash token as a bank denom, answered from GroveDB with real read-cost gas.\n");

    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    seed(&db);
    let q = GroveQuerier { db: db.clone() };

    let (alice, g1): (BalanceResponse, u64) = ask(&q, &balance_query("alice")).expect("alice ok");
    println!("## bank balance query (a Dash token as a denom)");
    println!("  alice udash balance = {} (gas {g1})", alice.amount.amount);
    assert_eq!(alice.amount.amount.u128(), 1000);
    assert_eq!(alice.amount.denom, "udash");
    assert!(g1 > 0, "gas is derived from the real GroveDB read cost");

    let (bob, _): (BalanceResponse, u64) = ask(&q, &balance_query("bob")).expect("bob ok");
    assert_eq!(bob.amount.amount.u128(), 500);
    let (carol, _): (BalanceResponse, u64) = ask(&q, &balance_query("carol")).expect("carol ok");
    assert_eq!(carol.amount.amount.u128(), 0, "a missing holder reads as zero");
    println!("  bob = {}, carol (absent) = {}", bob.amount.amount, carol.amount.amount);

    println!("\n## custom Dash query (token supply)");
    let (supply, g2): (SupplyResponse, u64) =
        ask(&q, &QueryRequest::Custom(DashQuery::TokenSupply { denom: "udash".to_string() })).expect("supply ok");
    println!("  udash total supply = {} (gas {g2})", supply.amount.amount);
    assert_eq!(supply.amount.amount.u128(), 1500);
    assert!(g2 > 0);

    // Determinism.
    let (alice2, g1b): (BalanceResponse, u64) = ask(&q, &balance_query("alice")).expect("alice2 ok");
    assert_eq!((alice2.amount.amount, g1b), (alice.amount.amount, g1), "queries are deterministic");

    // A corrupt stored balance is surfaced as an error, not read as zero.
    println!("\n## error surfacing");
    let corrupt: Result<(BalanceResponse, u64), String> = ask(&q, &balance_query("mallory"));
    assert!(corrupt.is_err(), "a corrupt balance surfaces an error rather than reading as zero");
    println!("  a corrupt stored balance surfaces an error: {}", corrupt.unwrap_err());

    println!("\n# Step 1 complete. The Querier answers a contract's reads against Dash-native token");
    println!("# state in GroveDB, with gas from the real read cost, missing keys as zero, and");
    println!("# corruption surfaced as an error rather than a false zero.");
}
