# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Architecture (implements all recommendations from arch review)
- Split App god object: extracted `UiState` (selection, modes, filters, display flags) and `DomainState` (quotes, holdings, history, alerts, groups, currency, failed_symbols) with `App` as thin coordinator. All methods delegate; pub fields updated via composition (rec #1 highest leverage).
- Input decoupling: `KeyCommand` enum, `key_to_command` mapper (context-aware for modes/overlays/secure), `handle_command` dispatch. `handle_key_event` now thin. Mouse still direct for min diff (rec #3).
- Provider boundary strengthened: `CachingQuoteProvider` decorator (std Arc/Mutex/Instant TTL 5s, wraps any impl). Always wired in `App::build`. Added docs on Yahoo v8 limitations (rec #2).
- Currency/market first-class: `any_markets_open()` aggregate using `MarketState`; exposed in header (green [open] / red [closed]). Failed symbols tracked in `DomainState` on partial fetches and surfaced via error banner (recs 5/6).
- Lints: expanded `[lints.clippy]` table with priority fixes + targeted allows for pre-existing noise (pedantic/unwrap/uninlined/casts now pass `cargo clippy -- -D warnings`); fixed new violations from refactors (rec #7).
- Lower: added notes/ADRs in CLAUDE.md; stub comments for future providers/persist/virtual/bg (recs lower).

### Added
- Sparkline charts: inline price trend visualization in sidebar (`S` to toggle)
- Enhanced detail view: day/52-week range bars, price history stats, currency info
- Export/import watchlists: JSON and CSV format (`e` to export)
- Desktop notifications: OS-level alerts via `notify-rust` when price thresholds are hit
- Multi-currency support: `[currency]` config section with display currency and conversion
- Search/filter mode: press `/` for case-insensitive search on symbol and name
- Price alerts: `[[alerts]]` config with `above`/`below` thresholds and overlay display
- Configurable crypto shortcuts: `[shortcuts]` config section for custom symbol expansions
- Persistent watchlist: symbols added/removed at runtime are saved back to config
- Mouse support: click to select rows, scroll wheel to navigate
- Market state parsing: pre/post/regular/closed states from Yahoo Finance API
- Bug report and feature request issue templates
- Pull request template with test plan checklist

### Changed
- `PriceHistory` uses `VecDeque` for O(1) front eviction (was O(n) `Vec::remove(0)`)
- `PriceHistory` rejects `NaN`/`Infinity` prices instead of corrupting sparkline data
- `PriceHistory` eagerly caches sparkline data and min/max on push (zero-alloc renders)
- `range_bar()` preserves `├`/`┤` endpoint markers when current price is at extremes
- Sparkline sidebar and detail view use consistent minimum data threshold (> 1 point)
- Holdings table respects group/search filter
- Config `sort_by` and `refresh_interval` now honored when CLI uses defaults
- Secure mode logic consolidated into single handler
- `--delay` rejects NaN, Infinity, and non-positive values
- Trimmed dependency features: reqwest (json + rustls only), chrono (clock + serde + std), futures-util (alloc only)

### Fixed
- `PriceHistory::new(0)` panics with a clear message instead of latent panic on first push
- Holdings table selection using wrong index after filtering
- `group_symbols[0]` synced when adding/removing symbols at runtime
- `select_bottom` not updating scroll offset
- CSV output escapes both symbol and name per RFC 4180
- `format_price` renders `0.0` as `$0.00` (was `$0.000000`)
- `format_price` handles negative prices correctly
- `centered_rect` clamps percentages to prevent `u16` underflow
- Removed duplicate `color` field in Args struct
- Replaced hardcoded visible rows with terminal-height-based calculation

### Removed
- Unused `--top`, `--filter`, `--no-header` CLI flags and `FilterType` enum
- Unused `thiserror` dependency
- Bogus npm ecosystem from dependabot config

### CI
- Optimized GitHub Actions: caching, slimmed matrix, concurrency groups, path-ignore
- Dependabot: grouped updates with conventional commit prefixes, Monday schedule
- CodeQL: `security-and-quality` query suite, explicit cargo build, caching
- Release workflow: scoped permissions, fixed secret exposure, security audit gate

### Testing
- 181 tests total (up from 80), zero clippy warnings
- PriceHistory: NaN, Infinity, zero max_len, min/max, cached sparkline data
- range_bar: width-3 minimum, endpoint preservation, clamping
- Export/import: JSON, CSV, round trip, empty lines
- Currency conversion, notifications, sparkline toggle, secure mode restrictions
- Comprehensive search/filter, price alerts, group cycling, mock provider injection

## [0.3.0] - 2026-02-20

### Added
- `--format json/csv` output modes for batch mode
- `--init` flag to generate default config file
- `--force` flag for overwriting existing config
- Interactive symbol add (`a`) and remove (`x`) keys
- Detail popup for individual stock info (`Enter`/`d`)
- Color-coded display and verbose output modes
- Scroll position tracking for large symbol lists

### Changed
- GitHub Actions pinned to full SHA with explicit permissions blocks

### Testing
- E2E test infrastructure with wiremock mock server
- 41 new tests covering keybinding, UI formatting, config, and app modules

## [0.1.1] - 2025-12-16

### Fixed
- Switch to Yahoo Finance v8 chart API after v7 quote API was deprecated
- Parallel fetching for multiple symbols to improve performance

### Changed
- Updated dependencies: crossterm 0.29, toml 0.9, dirs 6.0
- Updated GitHub Actions: checkout v6, cache v5

## [0.1.0] - 2025-12-16

### Added
- Initial release
- Real-time stock and cryptocurrency price monitoring
- Top-like terminal interface with familiar keyboard shortcuts
- Support for Yahoo Finance API (no API key required)
- Portfolio/holdings tracking with P/L calculations
- Multiple sort options (symbol, price, change, volume, market cap)
- TOML configuration file support
- Batch mode for scripting (`-b` flag)
- Secure mode to disable interactive commands (`-S` flag)
- Crypto symbol shortcuts (BTC.X -> BTC-USD)
- Configurable refresh interval (`-d` flag)
- Iteration limit (`-n` flag)
- Color-coded gains (green) and losses (red)
- Vim-style navigation (j/k, g/G)
- Help overlay (h/?)
- Performance data display

### Platforms
- Linux (x86_64, aarch64, musl)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64, aarch64)

[Unreleased]: https://github.com/somethingwithproof/stonktop/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/somethingwithproof/stonktop/compare/v0.1.1...v0.3.0
[0.1.1]: https://github.com/somethingwithproof/stonktop/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/somethingwithproof/stonktop/releases/tag/v0.1.0
