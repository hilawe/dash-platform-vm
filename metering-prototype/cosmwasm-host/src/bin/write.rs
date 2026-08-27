//! Module bindings, step 2: the write path, a contract-emitted message applied to Dash state.
//!
//! A real contract (hackatom `release`) queries its own token balance through the GroveDB-backed
//! Querier, then emits a `BankMsg::Send` of that balance to its beneficiary. A stand-in message
//! router applies the emitted message to Dash-native token balances in GroveDB, and the result is
//! proven, bound to the exact expected balances. The router VALIDATES the whole message before
//! applying it (known denom, sufficient funds), uses checked arithmetic, and applies all coins
//! atomically, so it cannot mint value or move the wrong asset.

use cosmwasm_host::{bank, gv, OverlayGroveStorage};
use cosmwasm_std::{
    from_json, to_json_binary, AllBalanceResponse, BalanceResponse, BankMsg, BankQuery, Binary, Coin,
    ContractResult, CosmosMsg, CustomQuery, Empty, QueryRequest, Response, SystemResult,
};
use cosmwasm_vm::testing::{mock_env, mock_info, MockApi};
use cosmwasm_vm::{
    call_execute, call_instantiate, Backend, BackendResult, GasInfo, Instance, InstanceOptions, Querier,
};
use grovedb::{Element, GroveDb};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const ROOT: &[&[u8]] = &[];
const CONTRACTS: &[u8] = b"contracts";
const CONTRACT_ID: &[u8] = b"cosmos2contract"; // matches MOCK_CONTRACT_ADDR
const DENOM: &str = bank::DENOM;

const HACKATOM: &[u8] = include_bytes!("../../testdata/hackatom.wasm");

/// Direct fixture seeding of a starting balance (NOT through the router): this is the initial
/// funding the demo starts from, not a transfer under test. Every transfer under test still goes
/// through the audited `bank::route_bank_send`.
fn seed_balance(db: &GroveDb, address: &str, amount: u128) {
    db.insert(
        [bank::BANK].as_ref(),
        &bank::bank_key(address),
        Element::new_item(amount.to_string().into_bytes()),
        None,
        None,
        gv(),
    )
    .unwrap()
    .expect("seed balance");
}

// ---- Querier backend over the bank state --------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
enum DashQuery {}
impl CustomQuery for DashQuery {}

struct GroveQuerier {
    db: Arc<GroveDb>,
}

impl Querier for GroveQuerier {
    fn query_raw(
        &self,
        request: &[u8],
        _gas_limit: u64,
    ) -> BackendResult<SystemResult<ContractResult<Binary>>> {
        // Charge for parsing the caller-controlled request on EVERY return path (never free gas for
        // unbounded input), on top of any read cost.
        let req_gas = cosmwasm_host::request_gas(request.len());
        let req: QueryRequest<DashQuery> = match from_json(request) {
            Ok(r) => r,
            Err(e) => {
                return (
                    Ok(SystemResult::Ok(ContractResult::Err(format!("bad query: {e}")))),
                    GasInfo::with_cost(req_gas),
                )
            }
        };
        // Every answer carries gas derived from the real GroveDB read cost, on BOTH the success and
        // error paths. A corrupt stored balance is surfaced as an error, never a false zero, and a
        // request for any denom other than this router's is rejected rather than being answered with
        // the udash balance mislabelled.
        let (binary, gas) = match req {
            QueryRequest::Bank(BankQuery::Balance { address, denom }) => {
                if denom != DENOM {
                    return (
                        Ok(SystemResult::Ok(ContractResult::Err(format!(
                            "this querier only serves denom {DENOM}, got {denom}"
                        )))),
                        GasInfo::with_cost(req_gas),
                    );
                }
                let (res, gas) = bank::read_balance_costed(&self.db, &address);
                let bal = match res {
                    Ok(b) => b,
                    Err(e) => {
                        return (
                            Ok(SystemResult::Ok(ContractResult::Err(e))),
                            GasInfo::with_cost(gas.saturating_add(req_gas)),
                        )
                    }
                };
                (
                    to_json_binary(&BalanceResponse { amount: Coin::new(bal, denom) }).unwrap(),
                    gas,
                )
            }
            QueryRequest::Bank(BankQuery::AllBalances { address }) => {
                let (res, gas) = bank::read_balance_costed(&self.db, &address);
                let bal = match res {
                    Ok(b) => b,
                    Err(e) => {
                        return (
                            Ok(SystemResult::Ok(ContractResult::Err(e))),
                            GasInfo::with_cost(gas.saturating_add(req_gas)),
                        )
                    }
                };
                let coins = if bal > 0 { vec![Coin::new(bal, DENOM)] } else { vec![] };
                (to_json_binary(&AllBalanceResponse { amount: coins }).unwrap(), gas)
            }
            _ => {
                return (
                    Ok(SystemResult::Ok(ContractResult::Err("unsupported query".to_string()))),
                    GasInfo::with_cost(req_gas),
                )
            }
        };
        (Ok(SystemResult::Ok(ContractResult::Ok(binary))), GasInfo::with_cost(gas.saturating_add(req_gas)))
    }
}

// ---- the stand-in node message router -----------------------------------------------------------

/// Map a contract-emitted `BankMsg::Send` onto the shared, audited bank router. The router reduces
/// the send to a net-delta map committed in one GroveDB transaction, so it cannot mint on a
/// self-transfer and cannot leave a partial application. `sender` is the emitting contract.
fn route_message(db: &GroveDb, sender: &str, msg: &CosmosMsg<Empty>) -> Result<String, String> {
    match msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            // Bound the coin list BEFORE the eager clone, matching the router's own bound, so an
            // arbitrarily long message cannot force unbounded cloning here.
            if amount.len() > bank::MAX_COINS_PER_SEND {
                return Err(format!("too many coins in one send: {}", amount.len()));
            }
            let coins: Vec<(String, u128)> =
                amount.iter().map(|c| (c.denom.clone(), c.amount.u128())).collect();
            // Enforce a storage-gas budget on the router's own reads/writes; a generous budget here
            // (adapter units), so the transfer commits, and the charged gas is surfaced in the note.
            bank::route_bank_send(db, sender, to_address, &coins, 100_000_000)
                .map(|(notes, gas)| format!("{notes} (bank storage gas {gas})"))
                .map_err(|(msg, _gas)| msg)
        }
        other => Err(format!("unhandled message: {other:?}")),
    }
}

fn main() {
    println!("# Module bindings, step 2: the write path. A real contract emits a transfer, and a");
    println!("# validating stand-in router applies it to Dash-native token balances in GroveDB.\n");

    let tmp = tempfile::TempDir::new().unwrap();
    let db = Arc::new(GroveDb::open(tmp.path()).unwrap());
    db.insert(ROOT, CONTRACTS, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contracts");
    db.insert([CONTRACTS].as_ref(), CONTRACT_ID, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("contract subtree");
    db.insert(ROOT, bank::BANK, Element::empty_tree(), None, None, gv())
        .unwrap()
        .expect("bank");
    seed_balance(&db, "cosmos2contract", 1000);

    println!("## before: contract balance = {}, beneficiary balance = {}",
        bank::read_balance(&db, "cosmos2contract").unwrap(), bank::read_balance(&db, "benefits").unwrap());

    let storage = OverlayGroveStorage::new(db.clone(), vec![CONTRACTS.to_vec(), CONTRACT_ID.to_vec()]);
    let backend = Backend {
        api: MockApi::default(),
        storage,
        querier: GroveQuerier { db: db.clone() },
    };
    let mut instance = Instance::from_code(
        HACKATOM,
        backend,
        InstanceOptions { gas_limit: u64::MAX, print_debug: false },
        None,
    )
    .expect("instantiate wasm");

    let env = mock_env();
    call_instantiate::<_, _, _, Empty>(&mut instance, &env, &mock_info("creator", &[]),
        br#"{"verifier": "verifies", "beneficiary": "benefits"}"#)
        .expect("call_instantiate")
        .into_result()
        .expect("instantiate ok");

    // Release AS the verifier: the contract queries its balance through our Querier and emits a
    // BankMsg::Send of it to the beneficiary.
    let response: Response<Empty> = call_execute::<_, _, _, Empty>(
        &mut instance, &env, &mock_info("verifies", &[]), br#"{"release":{}}"#,
    )
    .expect("call_execute")
    .into_result()
    .expect("release ok");

    // Flush the contract's own storage overlay on success (release also wrote nothing here, but the
    // pattern is uniform).
    // Route the success path through the ENFORCING commit (generous storage-gas budget in the
    // adapter's own units), the same budget-checked path a metered caller would use.
    instance.with_storage::<_, ()>(|s| { s.commit_within_budget(100_000_000).expect("commit within budget"); Ok(()) }).expect("with_storage");

    println!("## the contract emitted {} message(s) from release", response.messages.len());
    assert_eq!(response.messages.len(), 1, "release emits exactly one message");

    // Apply the emitted message through the validating router.
    for sub in &response.messages {
        let note = route_message(&db, "cosmos2contract", &sub.msg).expect("router applied the message");
        println!("  router applied: {note}");
    }

    let after_contract = bank::read_balance(&db, "cosmos2contract").unwrap();
    let after_benef = bank::read_balance(&db, "benefits").unwrap();
    println!("## after: contract balance = {after_contract}, beneficiary balance = {after_benef}");
    assert_eq!(after_contract, 0, "the contract's balance moved out");
    assert_eq!(after_benef, 1000, "the beneficiary received the transfer");
    // Conservation: total supply is unchanged (no minting).
    assert_eq!(after_contract + after_benef, 1000, "total supply is conserved by the transfer");

    // Prove the resulting bank state, bound to the EXACT set of balances (no extra keys permitted).
    let mut q = grovedb::Query::new();
    q.insert_all();
    let pq = grovedb::PathQuery {
        path: vec![bank::BANK.to_vec()],
        query: grovedb::SizedQuery { query: q, limit: None, offset: None },
    };
    let root = db.root_hash(None, gv()).unwrap().expect("root");
    let proof = db.prove_query(&pq, None, gv()).unwrap().expect("prove");
    let (vroot, results) = GroveDb::verify_query(&proof, &pq, gv()).expect("verify");
    assert_eq!(vroot, root, "the proof verifies against the live root");
    // Decode EVERY proven entry, requiring each to be a well-formed item (a non-item element is an
    // error, not silently skipped), and bind the whole set to the exact expected map.
    let mut proven: std::collections::BTreeMap<String, u128> = Default::default();
    for (_, k, el) in &results {
        match el {
            Some(Element::Item(v, _)) => {
                let key = String::from_utf8(k.clone()).expect("key is UTF-8");
                let addr = key
                    .strip_prefix(&format!("{DENOM}:"))
                    .unwrap_or_else(|| panic!("proven key {key:?} is not a {DENOM} balance"))
                    .to_string();
                let bal = bank::decode_balance(v).expect("proven balance decodes");
                proven.insert(addr, bal);
            }
            other => panic!("proven bank entry at {k:?} is not an item: {other:?}"),
        }
    }
    let expected: std::collections::BTreeMap<String, u128> =
        [("cosmos2contract".to_string(), 0u128), ("benefits".to_string(), 1000u128)]
            .into_iter()
            .collect();
    assert_eq!(proven, expected, "the proof binds the EXACT resulting bank state (no extra keys)");
    println!("## the resulting Dash token balances are provable and bound to the exact set:");
    for (a, b) in &proven {
        println!("  {a} = {b} {DENOM}");
    }

    println!("\n# Write path complete. A running contract read Dash-native state, emitted a transfer,");
    println!("# and the validating router moved the tokens in GroveDB (no minting, checked, atomic),");
    println!("# with the result provable and bound to the exact expected balances.");
}
