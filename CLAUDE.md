# CLAUDE.md

TUI application for real-time stock and cryptocurrency price monitoring (top-like interface).

## Stack
- Rust (edition 2021, MSRV 1.86)
- ratatui (terminal UI)
- crossterm (terminal backend)
- tokio (async runtime)

## Build & Test
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check  # Required by CI
```

## Usage
```bash
stonktop -s AAPL,GOOGL,BTC-USD
stonktop -s BTC,ETH -d 10    # 10s refresh
stonktop -b                   # Batch mode
```

## Notes
- Config: `~/.config/stonktop/config.toml`
- Data source: Yahoo Finance API
- Crypto shortcuts: `BTC` expands to `BTC-USD`
- CI enforces `cargo fmt --check`
