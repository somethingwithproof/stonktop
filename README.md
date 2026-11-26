# Stonktop

A top-like terminal UI for monitoring stock and cryptocurrency prices in real-time.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Features

- **Real-time Price Monitoring**: Live quotes for stocks and cryptocurrencies
- **Top-like Interface**: Familiar keyboard controls similar to `top`/`htop`
- **Sparkline Charts**: Inline price trend visualization in a sidebar
- **Portfolio Tracking**: Monitor your holdings with P/L calculations
- **Search & Filter**: Case-insensitive search across symbols and names
- **Price Alerts**: Desktop notifications when prices cross configured thresholds
- **Export/Import**: Save and load watchlists as JSON or CSV
- **Multi-Currency**: Display prices in any currency with live conversion rates
- **Multiple Sort Options**: Sort by symbol, price, change, volume, market cap
- **Batch Mode**: Non-interactive output for scripting (like `top -b`)
- **TOML Configuration**: Easy-to-edit config files
- **Color Coded**: Green for gains, red for losses
- **Crypto Shortcuts**: Use `BTC.X` or just `BTC` for `BTC-USD`

## Screenshots

```
STONKTOP - 8 symbols
3 up  4 down  1 unchanged  Updated: 5s ago

SYMBOL     NAME                  PRICE       CHANGE     CHG%        VOLUME      MKT CAP
NVDA       NVIDIA Corporation    $875.28     +12.45     +1.44%      45.2M       $2.16T
AAPL       Apple Inc.            $178.72     +2.31      +1.31%      52.1M       $2.78T
BTC-USD    Bitcoin USD           $43521.00   +521.00    +1.21%      28.5B       $852.1B
GOOGL      Alphabet Inc.         $141.80     +0.95      +0.67%      18.3M       $1.76T
MSFT       Microsoft Corporation $378.91     -1.23      -0.32%      22.4M       $2.81T
ETH-USD    Ethereum USD          $2245.50    -45.20     -1.97%      12.1B       $270.1B
AMZN       Amazon.com Inc.       $178.25     -3.42      -1.88%      31.2M       $1.85T
TSLA       Tesla Inc.            $248.50     -8.75      -3.40%      98.5M       $790.2B

 q:quit h:help s:sort r:reverse H:holdings f:fundamentals | Quotes | CHG% ▼ | Iter: 1
```

## Prerequisites

- **Rust**: Version 1.89 or higher (MSRV)
- **Cargo**: Included with Rust installation

## Installation

### From crates.io (Recommended)

```bash
cargo install stonktop
```

### Using cargo-binstall

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed, you can install pre-built binaries:

```bash
cargo binstall stonktop
```

### Pre-built Binaries

Download pre-built binaries from the [Releases](https://github.com/somethingwithproof/stonktop/releases) page.

Available platforms:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### From Source

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Watch specific symbols
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# With crypto shortcuts
stonktop -s AAPL,BTC,ETH,SOL
```

### Top-like Options

```bash
# Set refresh delay (like top -d)
stonktop -s AAPL,GOOGL -d 10

# Run N iterations then exit (like top -n)
stonktop -s AAPL -n 5

# Batch mode for scripting (like top -b)
stonktop -s AAPL,GOOGL -b

# Secure mode - disable interactive commands
stonktop -s AAPL -S
```

### Sorting Options

```bash
# Sort by price (descending)
stonktop -s AAPL,GOOGL -o price

# Sort by symbol (ascending)
stonktop -s AAPL,GOOGL -o symbol -r

# Available sort fields: symbol, name, price, change, change-percent, volume, market-cap
```

### Configuration File

```bash
# Use specific config file
stonktop -c ~/.config/stonktop/custom.toml

# Or place config at default location:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Command Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--symbols` | `-s` | Comma-separated list of symbols to watch |
| `--delay` | `-d` | Refresh delay in seconds (default: 5) |
| `--iterations` | `-n` | Number of iterations (0 = infinite) |
| `--batch` | `-b` | Batch mode - non-interactive output |
| `--secure` | `-S` | Secure mode - disable interactive commands |
| `--config` | `-c` | Path to configuration file |
| `--sort` | `-o` | Initial sort field |
| `--reverse` | `-r` | Reverse sort order |
| `--holdings` | `-H` | Show holdings/portfolio view |
| `--currency` | | Display currency (default: USD) |
| `--timeout` | | API timeout in seconds (default: 10) |
| `--format` | | Output format for batch mode: table, json, csv |
| `--color` | | Color output: auto, always, never |
| `--verbose` | `-v` | Verbose output |
| `--init` | | Generate a sample configuration file |
| `--force` | | Force overwrite existing config (with `--init`) |
| `--lang` | | Display language (`STONKTOP_LANG`, default: auto from `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Show help message |
| `--version` | `-V` | Show version |

## Supported Languages

Stonktop detects the display language from `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, and `LANG`, or accepts `--lang <LANG>`. All 48 supported locales include full CLI and TUI translations (including right-to-left alignment for Arabic, Persian, Hebrew, and Urdu) and localized documentation:

| Code | Language | README | Code | Language | README | Code | Language | README |
|------|----------|--------|------|----------|--------|------|----------|--------|
| `en` | English | [English](README.md) | `af-ZA` | Afrikaans | [Afrikaans](docs/i18n/af-ZA/README.md) | `bn` | বাংলা | [বাংলা](docs/i18n/bn/README.md) |
| `de` | Deutsch | [Deutsch](docs/i18n/de/README.md) | `am-ET` | አማርኛ | [አማርኛ](docs/i18n/am-ET/README.md) | `fa` | فارسی | [فارسی](docs/i18n/fa/README.md) |
| `es` | Español | [Español](docs/i18n/es/README.md) | `ar-EG` | العربية (مصر) | [العربية (مصر)](docs/i18n/ar-EG/README.md) | `he` | עברית | [עברית](docs/i18n/he/README.md) |
| `fr` | Français | [Français](docs/i18n/fr/README.md) | `ar-MA` | العربية (المغرب) | [العربية (المغرب)](docs/i18n/ar-MA/README.md) | `mr` | मराठी | [मराठी](docs/i18n/mr/README.md) |
| `it` | Italiano | [Italiano](docs/i18n/it/README.md) | `fr-MA` | Français (Maroc) | [Français (Maroc)](docs/i18n/fr-MA/README.md) | `ta` | தமிழ் | [தமிழ்](docs/i18n/ta/README.md) |
| `pt-BR` | Português (Brasil) | [Português](docs/i18n/pt-BR/README.md) | `fr-SN` | Français (Sénégal) | [Français (Sénégal)](docs/i18n/fr-SN/README.md) | `te` | తెలుగు | [తెలుగు](docs/i18n/te/README.md) |
| `nl` | Nederlands | [Nederlands](docs/i18n/nl/README.md) | `ha-NG` | Hausa | [Hausa](docs/i18n/ha-NG/README.md) | `th` | ไทย | [ไทย](docs/i18n/th/README.md) |
| `ru` | Русский | [Русский](docs/i18n/ru/README.md) | `ig-NG` | Igbo | [Igbo](docs/i18n/ig-NG/README.md) | `ur` | اردو | [اردو](docs/i18n/ur/README.md) |
| `uk` | Українська | [Українська](docs/i18n/uk/README.md) | `mg-MG` | Malagasy | [Malagasy](docs/i18n/mg-MG/README.md) | `ms` | Bahasa Melayu | [Bahasa Melayu](docs/i18n/ms/README.md) |
| `ja` | 日本語 | [日本語](docs/i18n/ja/README.md) | `rw-RW` | Ikinyarwanda | [Ikinyarwanda](docs/i18n/rw-RW/README.md) | `fil` | Filipino | [Filipino](docs/i18n/fil/README.md) |
| `ko` | 한국어 | [한국어](docs/i18n/ko/README.md) | `so-SO` | Soomaali | [Soomaali](docs/i18n/so-SO/README.md) | `bs` | Bosanski | [Bosanski](docs/i18n/bs/README.md) |
| `zh-CN` | 简体中文 | [简体中文](docs/i18n/zh-CN/README.md) | `sw-KE` | Kiswahili | [Kiswahili](docs/i18n/sw-KE/README.md) | `hr` | Hrvatski | [Hrvatski](docs/i18n/hr/README.md) |
| `zh-TW` | 繁體中文 | [繁體中文](docs/i18n/zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](docs/i18n/xh-ZA/README.md) | `sr` | Српски | [Српски](docs/i18n/sr/README.md) |
| `ar` | العربية | [العربية](docs/i18n/ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](docs/i18n/yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](docs/i18n/hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](docs/i18n/zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](docs/i18n/pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](docs/i18n/tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](docs/i18n/sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](docs/i18n/vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](docs/i18n/id/README.md) | | | | | | |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q`, `Esc` | Quit |
| `h`, `?` | Toggle help |
| `↑`, `k` | Move up |
| `↓`, `j` | Move down |
| `g`, `Home` | Go to top |
| `G`, `End` | Go to bottom |
| `PgUp` | Page up |
| `PgDn` | Page down |
| `s` | Cycle sort field |
| `r` | Reverse sort order |
| `1-7` | Sort by column |
| `Enter` | Show detail popup for selected symbol |
| `/` | Search/filter symbols |
| `a` | Add a symbol to watchlist |
| `d` | Remove selected symbol |
| `S` | Toggle sparkline charts sidebar |
| `e` | Export watchlist to JSON |
| `H` | Toggle holdings view |
| `f` | Toggle fundamentals |
| `Space`, `R` | Force refresh |
| `Tab` | Cycle symbol groups |

## Configuration

Create a TOML configuration file:

```toml
# ~/.config/stonktop/config.toml

[general]
refresh_interval = 5.0
timeout = 10
currency = "USD"

[watchlist]
symbols = [
    "AAPL",
    "GOOGL",
    "MSFT",
    "AMZN",
    "NVDA",
    "BTC-USD",
    "ETH-USD",
]

# Portfolio holdings (optional)
[[holdings]]
symbol = "AAPL"
quantity = 100
cost_basis = 150.00

[[holdings]]
symbol = "BTC-USD"
quantity = 0.5
cost_basis = 30000.00

[display]
show_header = true
show_fundamentals = false
show_holdings = false
sort_by = "change_percent"
sort_descending = true

[colors]
gain = "#00ff00"
loss = "#ff0000"
neutral = "#ffffff"
header = "#1e90ff"
border = "#444444"

# Symbol groups
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Multi-currency display (optional)
[currency]
display = "EUR"
convert = true

# Price alerts (optional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Custom crypto shortcuts (optional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Symbol Formats

| Format | Description | Example |
|--------|-------------|---------|
| Stock | Standard ticker | `AAPL`, `GOOGL` |
| Crypto | With USD suffix | `BTC-USD`, `ETH-USD` |
| Crypto shorthand | Expands to -USD | `BTC.X` -> `BTC-USD` |
| Crypto auto | Common cryptos | `BTC` -> `BTC-USD` |

## Data Source

Stonktop uses the Yahoo Finance API to fetch real-time quotes. No API key is required.

## Development

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Test

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Lint

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Format

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [ticker](https://github.com/achannarasappa/ticker)
- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Data from [Yahoo Finance](https://finance.yahoo.com/)
