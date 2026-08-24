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

### Docker integration & live E2E tests
Some integration and "live" E2E tests exercise the packaged app inside Docker (reproducible containerized runtime, tests the release artifact as users would run it).

- Requires Docker daemon.
- Build image: `docker build -t stonktop:test .`
- Run the ignored Docker tests: `cargo test --test integration_test test_docker_live_e2e -- --ignored`
- CI runs a dedicated `docker` job on every push/PR that builds the image and executes the live E2E verification.

See `tests/integration_test.rs` (test_docker_live_e2e) and `.github/workflows/ci.yml` (docker job) + the repo `Dockerfile`.

These complement the fast wiremock E2E (tests/e2e_test.rs) and pure CLI integration tests.


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
- App state: split implemented (UiState + DomainState composition; App thin coordinator). See arch review /tmp/grok-arch-review-stonktop.md recs #1-7 (all implemented on this branch/PR#71) + /tmp/grok-review-stonktop.md. KeyCommand for input, CachingQuoteProvider, market/failed tracking, lints hardened.
- Architecture decisions (2026): QuoteProvider remains sacred extension point (no direct Yahoo in App/UI). State split keeps ratatui immediate renders reading &App (delegates inside). Caching at boundary (TTL, best-effort). f64/Price pragmatic for display. anyhow for app errors. No new deps for cache (std only). Single crate kept (no workspace yet).
- See review /tmp/grok-review-stonktop.md and PR #71 for full list of implemented improvements (review fixes + "all" + "all arch recs")
