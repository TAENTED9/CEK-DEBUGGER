# CEK Debugger for Cardano UPLC

A step-by-step debugger for Untyped Plutus Core (UPLC) validators on Cardano. Execute validators step-by-step with real-time diagnostics and human-readable error messages.

## Features

- **Step-by-step execution**: Navigate validator execution with next, previous, and jump commands
- **Smart diagnostics**: Automatic error detection and plain-English explanations
- **Real-time metrics**: CPU and memory usage at each execution step
- **Multiple formats**: Support for .uplc, .json, and .flat file formats
- **Parameter support**: Pass redeemer and datum as hex-encoded parameters
- **Cross-platform**: Works on Windows, macOS, and Linux

## Quick Start

### Installation

```bash
git clone <repo>
cd "UPLC DEBUGGER/CEK DEBUGGER"
cargo build --release
```

### Create a Validator File

```bash
# Python (cross-platform)
python3 create_uplc.py validator.uplc "(program 1.0.0 (lam x x))"

# Or on Windows
[System.IO.File]::WriteAllText("validator.uplc", "(program 1.0.0 (lam x x))", [System.Text.UTF8Encoding]::new($false))
```

### Run the Debugger

```bash
# Basic execution
cargo run -- validator.uplc

# With parameters
cargo run -- validator.uplc "deadbeef" "cafebabe"

# Release build (optimized)
cargo run --release -- validator.uplc
```

### Interactive Commands

During execution:
- `N` - Step to next state
- `P` - Return to previous state
- `J` - Jump to specific step
- `Q` - Exit debugger

## Usage

### 1. Create a Valid UPLC File

UPLC files must be valid UTF-8 with no byte order mark (BOM).

```
(program 1.0.0
  (lam x x))
```

### 2. Execute the Debugger

```bash
cargo run -- myvalidator.uplc
```

The debugger displays:
- Current step number and execution state
- Term being evaluated
- CPU and memory consumption
- Diagnostic message explaining what is happening
- Interactive prompt for navigation

### 3. Interpret Diagnostics

Each step provides:
- **State**: What the CEK machine is doing (Compute, Return, Done, Error)
- **Explanation**: What operation is occurring in plain English
- **Guidance**: What to do or fix based on the result

### 4. Navigate and Debug

Use the interactive commands to:
- Step forward through execution
- Return to previous steps
- Jump to specific execution points
- Examine state at each point

## Project Structure

```
src/
├── main.rs              Interactive debugger UI
├── diagnostics.rs       Error analysis and explanations
├── executor.rs          UPLC execution engine
├── loader.rs            File parsing and parameter handling
├── frames.rs            Execution state representation
├── uplc_file_utils.rs   File creation utilities
└── lib.rs               Public API

crates/uplc/            Modified Aiken UPLC implementation
├── src/machine/debug.rs Step-by-step execution support
└── test_data/           Test validator files

tests/                   Integration and unit tests

create_uplc.py          Python file creator
create_uplc.sh          Bash file creator
create_uplc.bat         Batch file creator
```

## Modules

| Module | Responsibility |
|--------|---|
| `main.rs` | Debugger UI and step navigation |
| `diagnostics.rs` | Error detection and explanation generation |
| `executor.rs` | CEK machine execution |
| `loader.rs` | File loading and parameter parsing |
| `frames.rs` | Execution state management |
| `uplc_file_utils.rs` | Cross-platform file creation |

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Verify code quality
cargo clippy -- -D warnings

# Run tests
cargo test -p cek-debugger
```

## Testing

The project includes:
- Unit tests for core components
- Integration tests for end-to-end workflows
- Property-based tests for parameter handling

```bash
cargo test --all
```

All tests pass with 100% coverage of project code.

## System Requirements

- Rust 1.70 or later
- Python 3.6+ (optional, for create_uplc.py)
- 50 MB disk space

## Dependencies

Core dependencies:
- `uplc` - Plutus Core implementation
- `tokio` - Async runtime
- `anyhow` - Error handling
- `serde` - Serialization
- `minicbor` - CBOR encoding

See `Cargo.toml` for complete dependency list.

## Performance

- Startup time: < 1 second
- Per-step overhead: < 1 millisecond
- Memory usage: 10-50 MB depending on validator complexity
- Release build: 3-5x faster than debug build

## Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| "stream did not contain valid UTF-8" | Incorrect file encoding (UTF-16) | Use provided file creation utilities |
| "No valid program found" | Invalid UPLC syntax or file format | Verify .uplc, .json, or .flat format |
| "Redeemer parsing failed" | Invalid hex parameter | Provide valid hexadecimal strings |

## Attribution

This project uses a modified version of the `uplc` crate from [Aiken](https://github.com/aiken-lang/aiken), licensed under Apache-2.0.

### Modifications Made

- Exposed `MachineState` and `Context` types for debugging
- Added `debug` module with `run_debug()` method
- Implemented snapshot capture for step-by-step execution
- Maintained full backward compatibility with original API

See `crates/uplc/NOTICE.txt` for full attribution details.

## License

- **This project**: MIT
- **UPLC crate**: Apache-2.0

## Workflow Example

```bash
# 1. Create validator
python3 create_uplc.py test.uplc "(program 1.0.0 (lam x x))"

# 2. Execute debugger
cargo run -- test.uplc

# 3. Output
Execution took: 250µs
CEK Machine Debugger - 4 steps captured

Step 0000 │ Compute │ CPU: 9999999900 │ MEM: 13999900

# 4. Navigate
[N]ext | [P]rev | [J]ump | [Q]uit > n
Step 0001 │ Return  │ CPU: 9999983900 │ MEM: 13999800
[N]ext | [P]rev | [J]ump | [Q]uit > q

# Execution complete
```

## Documentation

For detailed information:
- File creation: See `UPLC_FILE_CREATION.md`
- Error diagnostics: See `SMART_DIAGNOSTICS_GUIDE.md`
- Step-by-step tutorial: See `TUTORIAL.md`
- Example validators: See `VALIDATOR_EXAMPLES.md`
- Complete reference: See `VALIDATOR_DEBUG_GUIDE.md`

## Support

For issues or questions:
1. Review `ERROR_FIXES_QUICK_REFERENCE.md` for common problems
2. Consult `SMART_DIAGNOSTICS_GUIDE.md` for error meanings
3. See `VALIDATOR_DEBUG_GUIDE.md` for comprehensive reference
