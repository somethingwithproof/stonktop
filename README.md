# Stonktop

A top-like terminal UI for monitoring stock and cryptocurrency prices in real-time.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/thomasvincent/stonktop/workflows/CI/badge.svg)](https://github.com/thomasvincent/stonktop/actions)

## Features

- **Real-time Price Monitoring**: Live quotes for stocks and cryptocurrencies
- **Top-like Interface**: Familiar keyboard controls similar to `top`/`htop`
- **Portfolio Tracking**: Monitor your holdings with P/L calculations
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

Download pre-built binaries from the [Releases](https://github.com/thomasvincent/stonktop/releases) page.

Available platforms:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### From Source

```bash
git clone https://github.com/thomasvincent/stonktop.git
cd stonktop
cargo build --release
```

The binary will be at `target/release/stonktop`.

### For Development

```bash
git clone https://github.com/thomasvincent/stonktop.git
cd stonktop
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
# - Linux/macOS: ~/.config/stonktop/config.toml
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
| `--top` | `-t` | Show only top N symbols |
| `--holdings` | `-H` | Show holdings/portfolio view |
| `--currency` | | Display currency (default: USD) |
| `--timeout` | | API timeout in seconds (default: 10) |
| `--verbose` | `-v` | Verbose output |
| `--help` | | Show help message |
| `--version` | `-V` | Show version |

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

## Building

### Requirements

- Rust 1.70 or later
- Cargo

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
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
