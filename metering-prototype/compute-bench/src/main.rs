//! Phase E of the metering prototype: an optional SAMPLED fuel calibration of the COMPUTE dimension
//! of terminal work.
//!
//! Phases A, C, and D measured the STORAGE dimensions of terminal work. The design's terminal-work
//! vector also has a compute component (a VM program's settlement logic), which those phases could
//! not price without a VM. Phase E calibrates it the way the design's own runtime choice implies: it
//! runs a fixed integer workload through an off-the-shelf deterministic WebAssembly interpreter
//! (wasmi) with FUEL metering ON, and reports the fuel consumed. Fuel is a deterministic count of
//! executed operations, not a wall-clock time, so it is a consensus-safe unit, which is exactly
//! what a metered compute dimension needs.
//!
//! What this establishes: the compute dimension has a real, deterministic, measurable unit (fuel),
//! it is reproducible run to run, and it scales linearly with the work done, so a class's compute
//! terminal-work component can be denominated in fuel just as its storage component is denominated
//! in bytes. This is a SAMPLED per-step unit and its scaling law for ONE representative kernel, not a
//! worst-case bound over arbitrary programs (that needs a full opcode-cost model) and not a claim
//! about absolute VM speed; wall-clock is reported only as context and is not the unit.

use wasmi::{Config, Engine, Linker, Module, Store};
use std::time::Instant;

/// A fixed integer workload: an LCG-style mixing loop of `n` iterations over 64-bit integers. This
/// stands in for the deterministic integer settlement math a terminalization might run (the design
/// enforces integer-only execution). Fuel consumed is linear in `n`.
const WORKLOAD_WAT: &str = r#"
(module
  (func (export "run") (param $n i64) (result i64)
    (local $i i64)
    (local $acc i64)
    (local.set $acc (i64.const 1))
    (block $exit
      (loop $loop
        (br_if $exit (i64.ge_u (local.get $i) (local.get $n)))
        (local.set $acc
          (i64.add
            (i64.mul (local.get $acc) (i64.const 6364136223846793005))
            (i64.const 1442695040888963407)))
        (local.set $i (i64.add (local.get $i) (i64.const 1)))
        (br $loop)))
    (local.get $acc)))
"#;

/// Run the workload for `n` iterations under fuel metering and return (fuel_consumed, wall_nanos).
fn run_workload(n: i64) -> (u64, u128) {
    let mut config = Config::default();
    config.consume_fuel(true);
    let engine = Engine::new(&config);
    let wasm = wat::parse_str(WORKLOAD_WAT).expect("compile WAT");
    let module = Module::new(&engine, &wasm[..]).expect("load module");
    let mut store = Store::new(&engine, ());
    // Grant a large fuel budget; we measure how much of it the run consumes.
    store.add_fuel(100_000_000_000).expect("add fuel");
    let linker = Linker::new(&engine);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate")
        .start(&mut store)
        .expect("start");
    let run = instance
        .get_typed_func::<i64, i64>(&store, "run")
        .expect("typed func");

    let start = Instant::now();
    let _result = run.call(&mut store, n).expect("call run");
    let wall = start.elapsed().as_nanos();
    let consumed = store.fuel_consumed().expect("fuel consumed");
    (consumed, wall)
}

fn main() {
    println!("# Phase E: compute-dimension fuel calibration via a deterministic Wasm interpreter (wasmi, fuel)");
    println!("# Fuel is a deterministic executed-operation count, not wall-clock. It is the unit;");
    println!("# wall-clock is context only. Each size is run twice and required identical fuel.\n");

    // Sizes spanning the range a terminal settlement might do, from a trivial fixed recipe to a
    // large fan-out distribution loop.
    let sizes: &[i64] = &[0, 1, 8, 64, 256, 1024, 8192];

    println!("{:>8}  {:>14}  {:>12}  {:>14}", "iters", "fuel", "fuel/iter", "wall_ns");
    let mut points: Vec<(i64, u64)> = Vec::new();
    for &n in sizes {
        let (fuel1, _w1) = run_workload(n);
        let (fuel2, wall) = run_workload(n);
        assert_eq!(fuel1, fuel2, "fuel must be deterministic for n={n}");
        let per_iter = if n > 0 { fuel1 as f64 / n as f64 } else { 0.0 };
        println!("{n:>8}  {fuel1:>14}  {per_iter:>12.2}  {wall:>14}");
        points.push((n, fuel1));
    }

    // ASSERT exact affine linearity: fuel(n) = base + slope * n across EVERY measured size, not just
    // repeatability. Derive the slope and base from two large points, then require every measured
    // point to match exactly. This turns "the fuel is linear" from a printed claim into a checked
    // one; a deterministic-but-nonlinear schedule would fail here.
    let point = |target: i64| -> u64 {
        points.iter().find(|(n, _)| *n == target).expect("size measured").1
    };
    let (f_big, f_mid) = (point(8192), point(1024));
    let span = (8192 - 1024) as u64;
    assert_eq!((f_big - f_mid) % span, 0, "the fuel slope is integral");
    let slope = (f_big - f_mid) / span;
    let base = f_big - slope * 8192u64;
    for (n, fuel) in &points {
        assert_eq!(
            *fuel,
            base + slope * (*n as u64),
            "fuel is exactly affine base+slope*n at n={n} (base={base}, slope={slope})"
        );
    }

    println!();
    println!("# Fuel is exactly affine: fuel(n) = {base} + {slope} * n, verified at every measured size.");
    println!("# The compute unit is {slope} fuel per integer settlement step, with a fixed per-call");
    println!("# overhead of {base} fuel. So a class's compute terminal-work component is (settlement");
    println!("# steps) times {slope} fuel plus the fixed overhead.");
    println!("#");
    println!("# CLAIM WIDTH: this is the SAMPLED per-step cost of ONE representative integer kernel,");
    println!("# not a worst-case bound over arbitrary programs. A per-step worst-case bound needs the");
    println!("# full opcode cost model; what is established here is a deterministic, reproducible,");
    println!("# exactly-linear unit and its scaling law for this kernel.");
}
