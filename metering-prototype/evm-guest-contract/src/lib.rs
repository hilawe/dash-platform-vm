//! A minimal EVM interpreter packaged as a CosmWasm contract, for the "EVM as guest" spike.
//!
//! This is a real, if tiny, EVM: a 256-bit stack machine that executes EVM bytecode and whose
//! SSTORE and SLOAD go to the CosmWasm contract's own key-value storage (`deps.storage`). When this
//! contract runs inside cosmwasm-vm over a GroveDB-backed storage backend, an EVM `SSTORE` therefore
//! lands in GroveDB, which is the whole point of the "EVM as a guest over GroveDB-backed CosmWasm
//! storage" demonstration. The interpreter supports the opcodes a storage demonstration needs
//! (PUSH1..PUSH32, ADD, POP, SLOAD, SSTORE, STOP), all integer-only, so the compiled Wasm has no
//! floats and passes the VM's contract validation.

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Run a hex-encoded EVM bytecode program. Its SSTOREs land in contract storage.
    Run { code: String },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Read the 32-byte EVM storage word at the given slot.
    Slot { key: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct SlotResponse {
    /// The 32-byte word, hex-encoded.
    pub value: String,
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("evm_guest", "instantiated"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Run { code } => {
            let bytecode = hex_decode(&code)?;
            let steps = run_evm(deps.storage, &bytecode)?;
            Ok(Response::new()
                .add_attribute("evm_guest", "ran")
                .add_attribute("opcodes", steps.to_string()))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Slot { key } => {
            let slot = u256_from_u64(key);
            let word = deps.storage.get(&slot).unwrap_or_else(|| vec![0u8; 32]);
            to_json_binary(&SlotResponse {
                value: hex_encode(&word),
            })
        }
    }
}

/// Execute EVM bytecode against contract storage. Returns the number of opcodes executed.
fn run_evm(storage: &mut dyn Storage, code: &[u8]) -> StdResult<u32> {
    let mut stack: Vec<[u8; 32]> = Vec::new();
    let mut pc = 0usize;
    let mut steps = 0u32;
    while pc < code.len() {
        let op = code[pc];
        pc += 1;
        steps += 1;
        match op {
            0x00 => break, // STOP
            0x01 => {
                // ADD
                let a = pop(&mut stack)?;
                let b = pop(&mut stack)?;
                stack.push(add256(a, b));
            }
            0x50 => {
                // POP
                pop(&mut stack)?;
            }
            0x54 => {
                // SLOAD: key = pop; push storage[key]
                let key = pop(&mut stack)?;
                let word = storage.get(&key).unwrap_or_else(|| vec![0u8; 32]);
                let mut w = [0u8; 32];
                let n = core::cmp::min(32, word.len());
                w[32 - n..].copy_from_slice(&word[..n]);
                push(&mut stack, w)?;
            }
            0x55 => {
                // SSTORE: key = pop; value = pop. Ethereum semantics: storing zero CLEARS the slot
                // (the slot becomes absent), which keeps the persisted state and its proof shape in
                // line with a real EVM store rather than persisting a 32-byte zero word.
                let key = pop(&mut stack)?;
                let value = pop(&mut stack)?;
                if value == [0u8; 32] {
                    storage.remove(&key);
                } else {
                    storage.set(&key, &value);
                }
            }
            0x60..=0x7f => {
                // PUSH1..PUSH32
                let n = (op - 0x60 + 1) as usize;
                if pc + n > code.len() {
                    return Err(StdError::generic_err("truncated PUSH"));
                }
                let mut w = [0u8; 32];
                w[32 - n..].copy_from_slice(&code[pc..pc + n]);
                push(&mut stack, w)?;
                pc += n;
            }
            other => {
                return Err(StdError::generic_err(format!(
                    "unsupported opcode 0x{other:02x}"
                )));
            }
        }
    }
    Ok(steps)
}

fn pop(stack: &mut Vec<[u8; 32]>) -> StdResult<[u8; 32]> {
    stack.pop().ok_or_else(|| StdError::generic_err("stack underflow"))
}

/// Push, enforcing the EVM's 1024-element stack limit.
fn push(stack: &mut Vec<[u8; 32]>, word: [u8; 32]) -> StdResult<()> {
    if stack.len() >= 1024 {
        return Err(StdError::generic_err("stack overflow (limit 1024)"));
    }
    stack.push(word);
    Ok(())
}

/// 256-bit big-endian addition with wraparound.
fn add256(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut carry = 0u16;
    let mut i = 32;
    while i > 0 {
        i -= 1;
        let s = a[i] as u16 + b[i] as u16 + carry;
        out[i] = (s & 0xff) as u8;
        carry = s >> 8;
    }
    out
}

fn u256_from_u64(x: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&x.to_be_bytes());
    w
}

fn hex_decode(s: &str) -> StdResult<Vec<u8>> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let b = s.as_bytes();
    if b.len() % 2 != 0 {
        return Err(StdError::generic_err("odd-length hex"));
    }
    let mut out = Vec::with_capacity(b.len() / 2);
    let mut i = 0;
    while i < b.len() {
        out.push((hex_val(b[i])? << 4) | hex_val(b[i + 1])?);
        i += 2;
    }
    Ok(out)
}

fn hex_val(c: u8) -> StdResult<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(StdError::generic_err("bad hex digit")),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &x in bytes {
        s.push(nib(x >> 4));
        s.push(nib(x & 0x0f));
    }
    s
}

fn nib(n: u8) -> char {
    if n < 10 {
        (b'0' + n) as char
    } else {
        (b'a' + n - 10) as char
    }
}
