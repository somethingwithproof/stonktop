# CLAUDE.md

TUI application for real-time stock and cryptocurrency price monitoring (top-like interface).

## Stack
- Rust (edition 2021, MSRV 1.88)
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
- Data source: Yahoo Finance API (v8 chart; see api.rs for validation + basic retry/backoff)
- Crypto shortcuts: `BTC` expands to `BTC-USD`
- CI enforces `cargo fmt --check`
- Currency conversion: fully wired (see App::refresh, convert_price, UI table support for display_currency)
- Market status: uses market_state from quotes for awareness (header can show open/closed)
- Lints: [lints] in Cargo.toml (unsafe forbid, clippy warn pedantic/unwrap)
- Price values: use `Price` alias (f64) for quote/display values; see models.rs for rationale vs decimal
- Partial API errors: best-effort, tracked for UX
- Dead code cleaned for color mode (feature implemented via ColorMode + UiColors)
- App state: large; consider further splits for new features
- See review /tmp/grok-review-stonktop.md and PR #71 for full list of implemented improvements
