# CEK Debugger for Cardano UPLC

A step-by-step debugger for Untyped Plutus Core (UPLC) scripts on Cardano.

## Attribution

This project includes a modified version of the `uplc` crate from the [Aiken](https://github.com/aiken-lang/aiken) project, licensed under Apache-2.0. See `crates/uplc/NOTICE.txt` for details.

## Modifications to uplc Crate

This document tracks all modifications made to the original Aiken uplc crate.

## Files Modified

### src/machine.rs

- Changed `MachineState` enum from private to `pub(crate)` (line ~XX)
- Changed `Context` enum from private to `pub(crate)` (line ~XX)
- Added `pub mod debug;` export

## Files Added

### src/machine/debug.rs

- New module for debugging support
- Exports `StepSnapshot` struct
- Implements `Machine::run_debug()` method for step-by-step execution
- Helper functions: `capture_snapshot()`, `pretty_value()`, `count_context_depth()`

## Why These Changes?

The original uplc crate's `Machine::run()` method evaluates UPLC programs to completion without exposing intermediate states. For debugging Cardano validators, we need visibility into each evaluation step (the CEK machine's Control, Environment, and Continuation).

These modifications maintain backward compatibility - existing code using `Machine::run()` continues to work unchanged.
