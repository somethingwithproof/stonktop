# Stonktop

> [English](../../../README.md)

Isang terminal UI na kahawig ng `top` para sa real-time na pagsubaybay ng mga presyo ng stock at cryptocurrency.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Mga Tampok

- **Real-time na Pagsubaybay ng Presyo**: Mga live na presyo para sa mga stock at cryptocurrency
- **Interface na Kahawig ng `top`**: Pamilyar na mga kontrol sa keyboard tulad ng `top`/`htop`
- **Mga Sparkline Chart**: Inline na visualisasyon ng trend ng presyo sa isang sidebar
- **Pagsubaybay ng Portfolio**: Subaybayan ang iyong mga hawak na asset na may pagkalkula ng kita/pagkalugi (P/L)
- **Paghahanap at Pag-filter**: Paghahanap na hindi sensitibo sa malaki o maliit na titik sa mga simbolo at pangalan
- **Mga Alerto sa Presyo**: Mga abiso sa desktop kapag lumampas ang presyo sa nakatakdang hangganan
- **Pag-export/Pag-import**: I-save at i-load ang mga watchlist bilang JSON o CSV
- **Maramihang Pera**: Ipakita ang mga presyo sa anumang pera gamit ang live na halaga ng palitan
- **Maramihang Opsyon sa Pag-uuri**: Mag-uri ayon sa simbolo, presyo, pagbabago, volume, at market cap
- **Batch Mode**: Hindi interaktibong output para sa pag-script (tulad ng `top -b`)
- **Konpigurasyong TOML**: Mga file ng konpigurasyon na madaling i-edit
- **May Kulay na Indikasyon**: Berde para sa pagtaas, pula para sa pagbaba
- **Mga Shortcut sa Crypto**: Gamitin ang `BTC.X` o `BTC` lamang para sa `BTC-USD`

## Mga Screenshot

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

## Mga Kinakailangan

- **Rust**: Bersyon 1.88 o mas bago (MSRV)
- **Cargo**: Kasama sa pag-install ng Rust

## Pag-install

### Mula sa crates.io (Inirerekomenda)

```bash
cargo install stonktop
```

### Gamit ang cargo-binstall

Kung naka-install ang [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), maaari kang mag-install ng mga pre-built na binary:

```bash
cargo binstall stonktop
```

### Mga Pre-built na Binary

Mag-download ng mga pre-built na binary mula sa pahina ng [Releases](https://github.com/somethingwithproof/stonktop/releases).

Mga available na platform:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Mula sa Source Code

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Paggamit

### Pangunahing Paggamit

```bash
# Subaybayan ang mga partikular na simbolo
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Gamit ang mga shortcut sa crypto
stonktop -s AAPL,BTC,ETH,SOL
```

### Mga Opsyon na Kahawig ng top

```bash
# Itakda ang pagitan ng pag-refresh (tulad ng top -d)
stonktop -s AAPL,GOOGL -d 10

# Magpatakbo ng N na ulit pagkatapos ay lumabas (tulad ng top -n)
stonktop -s AAPL -n 5

# Batch mode para sa pag-script (tulad ng top -b)
stonktop -s AAPL,GOOGL -b

# Secure mode - i-disable ang mga interaktibong command
stonktop -s AAPL -S
```

### Mga Opsyon sa Pag-uuri

```bash
# Mag-uri ayon sa presyo (pababa)
stonktop -s AAPL,GOOGL -o price

# Mag-uri ayon sa simbolo (pataas)
stonktop -s AAPL,GOOGL -o symbol -r

# Mga available na field sa pag-uuri: symbol, name, price, change, change-percent, volume, market-cap
```

### File ng Konpigurasyon

```bash
# Gumamit ng partikular na file ng konpigurasyon
stonktop -c ~/.config/stonktop/custom.toml

# O ilagay ang konpigurasyon sa default na lokasyon:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Mga Opsyon sa Command Line

| Opsyon | Maikli | Paglalarawan |
|--------|--------|--------------|
| `--symbols` | `-s` | Listahan ng mga simbolong paghihiwalayin ng kuwit na susubaybayan |
| `--delay` | `-d` | Pagitan ng pag-refresh sa segundo (default: 5) |
| `--iterations` | `-n` | Bilang ng mga ulit (0 = walang hangganan) |
| `--batch` | `-b` | Batch mode — hindi interaktibong output |
| `--secure` | `-S` | Secure mode — i-disable ang mga interaktibong command |
| `--config` | `-c` | Path patungo sa file ng konpigurasyon |
| `--sort` | `-o` | Paunang field ng pag-uuri |
| `--reverse` | `-r` | Baliktarin ang pagkakasunod-sunod ng pag-uuri |
| `--holdings` | `-H` | Ipakita ang view ng mga holdings/portfolio |
| `--currency` | | Perang ipapakita (default: USD) |
| `--timeout` | | Timeout ng API sa segundo (default: 10) |
| `--format` | | Format ng output para sa batch mode: `table`, `json`, `csv` |
| `--color` | | Kulay ng output: `auto`, `always`, `never` |
| `--verbose` | `-v` | Detalyadong output |
| `--init` | | Bumuo ng halimbawang file ng konpigurasyon |
| `--force` | | Sapilitang i-overwrite ang kasalukuyang konpigurasyon (kasama ng `--init`) |
| `--lang` | | Wika ng display (`STONKTOP_LANG`, default: awtomatiko mula sa `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Ipakita ang mensahe ng tulong |
| `--version` | `-V` | Ipakita ang bersyon |

## Mga Sinusuportahang Wika

Awtomatikong natutukoy ng Stonktop ang wika ng display mula sa `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, at `LANG`, o tumatanggap ng `--lang <LANG>`. Lahat ng 45 sinusuportahang locale ay may kumpletong salin sa CLI at TUI (kabilang ang kanan-pakaliwang pagkakahanay para sa Arabic, Persian, Hebrew, at Urdu) at naisalokal na dokumentasyon:

| Code | Wika | README | Code | Wika | README | Code | Wika | README |
|------|------|--------|------|------|--------|------|------|--------|
| `en` | English | [English](../../../README.md) | `af-ZA` | Afrikaans | [Afrikaans](../af-ZA/README.md) | `bn` | বাংলা | [বাংলা](../bn/README.md) |
| `de` | Deutsch | [Deutsch](../de/README.md) | `am-ET` | አማርኛ | [አማርኛ](../am-ET/README.md) | `fa` | فارسی | [فارسی](../fa/README.md) |
| `es` | Español | [Español](../es/README.md) | `ar-EG` | العربية (مصر) | [العربية (مصر)](../ar-EG/README.md) | `he` | עברית | [עברית](../he/README.md) |
| `fr` | Français | [Français](../fr/README.md) | `ar-MA` | العربية (المغرب) | [العربية (المغرب)](../ar-MA/README.md) | `mr` | मराठी | [मराठी](../mr/README.md) |
| `it` | Italiano | [Italiano](../it/README.md) | `fr-MA` | Français (Maroc) | [Français (Maroc)](../fr-MA/README.md) | `ta` | தமிழ் | [தமிழ்](../ta/README.md) |
| `pt-BR` | Português (Brasil) | [Português](../pt-BR/README.md) | `fr-SN` | Français (Sénégal) | [Français (Sénégal)](../fr-SN/README.md) | `te` | తెలుగు | [తెలుగు](../te/README.md) |
| `nl` | Nederlands | [Nederlands](../nl/README.md) | `ha-NG` | Hausa | [Hausa](../ha-NG/README.md) | `th` | ไทย | [ไทย](../th/README.md) |
| `ru` | Русский | [Русский](../ru/README.md) | `ig-NG` | Igbo | [Igbo](../ig-NG/README.md) | `ur` | اردو | [اردو](../ur/README.md) |
| `uk` | Українська | [Українська](../uk/README.md) | `mg-MG` | Malagasy | [Malagasy](../mg-MG/README.md) | `ms` | Bahasa Melayu | [Bahasa Melayu](../ms/README.md) |
| `ja` | 日本語 | [日本語](../ja/README.md) | `rw-RW` | Ikinyarwanda | [Ikinyarwanda](../rw-RW/README.md) | `fil` | Filipino | [Filipino](../fil/README.md) |
| `ko` | 한국어 | [한국어](../ko/README.md) | `so-SO` | Soomaali | [Soomaali](../so-SO/README.md) | | | |
| `zh-CN` | 简体中文 | [简体中文](../zh-CN/README.md) | `sw-KE` | Kiswahili | [Kiswahili](../sw-KE/README.md) | | | |
| `zh-TW` | 繁體中文 | [繁體中文](../zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](../xh-ZA/README.md) | | | |
| `ar` | العربية | [العربية](../ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](../yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](../hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](../zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](../pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](../tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](../sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](../vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](../id/README.md) | | | | | | |

## Mga Keyboard Shortcut

| Key | Aksyon |
|-----|--------|
| `q`, `Esc` | Lumabas |
| `h`, `?` | I-toggle ang tulong |
| `↑`, `k` | Umakyat |
| `↓`, `j` | Bumaba |
| `g`, `Home` | Pumunta sa itaas |
| `G`, `End` | Pumunta sa ibaba |
| `PgUp` | Umakyat ng isang pahina |
| `PgDn` | Bumaba ng isang pahina |
| `s` | Magpalit-palit ng field sa pag-uuri |
| `r` | Baliktarin ang pagkakasunod-sunod ng pag-uuri |
| `1-7` | Mag-uri ayon sa kolum |
| `Enter`, `d` | Ipakita ang popup ng detalye para sa napiling simbolo |
| `/` | Maghanap/mag-filter ng mga simbolo |
| `a` | Magdagdag ng simbolo sa watchlist |
| `x` | Alisin ang napiling simbolo |
| `S` | I-toggle ang sidebar ng mga sparkline chart |
| `e` | I-export ang watchlist sa JSON |
| `H` | I-toggle ang view ng mga holdings |
| `f` | I-toggle ang mga fundamentals |
| `Space`, `R` | Sapilitang mag-refresh |
| `Tab` | Magpalit-palit ng mga pangkat ng simbolo |

## Konpigurasyon

Gumawa ng file ng konpigurasyong TOML:

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

# Mga holdings ng portfolio (opsyonal)
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

# Mga pangkat ng simbolo
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Display sa maramihang pera (opsyonal)
[currency]
display = "EUR"
convert = true

# Mga alerto sa presyo (opsyonal)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Mga custom na shortcut sa crypto (opsyonal)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Mga Format ng Simbolo

| Format | Paglalarawan | Halimbawa |
|--------|--------------|-----------|
| Stock | Karaniwang ticker | `AAPL`, `GOOGL` |
| Crypto | May hulaping USD | `BTC-USD`, `ETH-USD` |
| Maikling crypto | Pinalalawak sa `-USD` | `BTC.X` -> `BTC-USD` |
| Awtomatikong crypto | Mga karaniwang cryptocurrency | `BTC` -> `BTC-USD` |

## Pinagmulan ng Data

Gumagamit ang Stonktop ng Yahoo Finance API upang kumuha ng mga real-time na presyo. Walang kinakailangang API key.

## Pag-develop

### Pag-build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Pagsusuri

```bash
# Patakbuhin ang lahat ng pagsusuri
cargo test

# Patakbuhin ang mga pagsusuri na may output
cargo test -- --nocapture
```

### Pagsusuri ng Code (Lint)

```bash
# Suriin para sa mga isyu sa linting
cargo clippy --all-targets --all-features

# Ilapat ang mga awtomatikong pag-aayos
cargo clippy --fix --allow-dirty
```

### Pag-format

```bash
# Suriin ang pag-format
cargo fmt -- --check

# Ilapat ang pag-format
cargo fmt
```

## Pag-ambag

Malugod na tinatanggap ang mga kontribusyon! Huwag mag-atubiling magsumite ng Pull Request.

1. I-fork ang repository
2. Gumawa ng iyong feature branch (`git checkout -b feature/amazing-feature`)
3. I-commit ang iyong mga pagbabago (`git commit -m 'Add amazing feature'`)
4. I-push sa branch (`git push origin feature/amazing-feature`)
5. Magbukas ng Pull Request

## Lisensya

Ang proyektong ito ay lisensyado sa ilalim ng MIT License — tingnan ang file na [LICENSE](../../../LICENSE) para sa mga detalye.

## Mga Pagkilala

- Hango sa [ticker](https://github.com/achannarasappa/ticker)
- Binuo gamit ang [ratatui](https://github.com/ratatui-org/ratatui)
- Data mula sa [Yahoo Finance](https://finance.yahoo.com/)
