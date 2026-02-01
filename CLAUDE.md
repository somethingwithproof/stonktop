# Stonktop

## Purpose
Top-like terminal UI for real-time stock and cryptocurrency price monitoring.

## Stack
- Rust (edition 2021, MSRV 1.70)
- TUI: ratatui + crossterm
- Published on crates.io

## Build & Test
```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings -W clippy::pedantic
cargo deny check
```

## Structure
- `src/` - application source
- `tests/` - integration tests
- `scripts/` - build/release scripts

## Standards
- `clippy` pedantic clean
- `Result`/`Option` for all fallible operations
- TOML configuration files
- Conventional Commits: `type(scope): description`

## Conventions
- Keyboard controls mirror `top`/`htop`
- Batch mode (`-b`) for scripting
- Crypto shortcuts: `BTC.X` or `BTC` resolves to `BTC-USD`
