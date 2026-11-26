# Stonktop

> [English](../../../README.md)

Manhajar amfani ta tasha (terminal UI) mai kama da `top` domin sa ido kan farashin hannun jari da kuɗaɗen dijital (cryptocurrency) kai tsaye.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Fasaloli

- **Sa Ido kan Farashi Kai Tsaye**: Sabbin farashin hannun jari da kuɗaɗen dijital a ainihin lokaci
- **Tsarin Fuska Mai Kama da `top`**: Maɓallan sarrafawa da aka saba da su kamar `top`/`htop`
- **Ƙananan Jadawalin Sparkline**: Nuna yanayin hauhawa ko saukar farashi a ɓangaren gefe
- **Bibiyar Kundin Hannun Jari (Portfolio)**: Sa ido kan kadarorinka tare da ƙididdigar riba da asara (P/L)
- **Bincike da Tacewa**: Bincike mai sauri ba tare da la'akari da girman haruffa ba a cikin alamomi da sunaye
- **Sanarwar Farashi**: Saƙonnin faɗakarwa a allon kwamfuta idan farashi ya wuce iyakar da aka saita
- **Fitarwa da Shigowa**: Ajiye da ɗauko jerin sa ido a tsarin JSON ko CSV
- **Kuɗaɗe Masu Yawa**: Nuna farashi a kowace irin kuɗi tare da canjin farashin musaya kai tsaye
- **Zaɓuɓɓukan Jeri Masu Yawa**: Jera bisa alama, farashi, sauyi, yawan ciniki, ko ƙimar kasuwa
- **Yanayin Rukuni (Batch Mode)**: Fitar da bayanai ba tare da mu'amala ba domin rubutun sarrafawa ta atomatik (kamar `top -b`)
- **Saitin TOML**: Fayilolin saiti masu sauƙin gyara
- **Rarrabewar Launuka**: Kore don riba, ja don asara
- **Gajerun Hanyoyin Kuɗaɗen Dijital**: Yi amfani da `BTC.X` ko `BTC` kawai maimakon `BTC-USD`

## Hotunan Allo

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

## Abubuwan da ake Buƙata

- **Rust**: Sigar 1.89 ko sama da haka (MSRV)
- **Cargo**: Yana zuwa tare da shigar da Rust

## Shigarwa

### Daga crates.io (An fi ba da Shawara)

```bash
cargo install stonktop
```

### Amfani da cargo-binstall

Idan kana da [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) a kwamfutarka, za ka iya shigar da shirye-shiryen da aka riga aka gina:

```bash
cargo binstall stonktop
```

### Fayilolin da aka Riga aka Gina

Sauke fayilolin da aka riga aka gina daga shafin [Releases](https://github.com/somethingwithproof/stonktop/releases).

Tsare-tsaren da ake samu:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Daga Asalin Lamba

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Amfani

### Amfani na Asali

```bash
# Sa ido kan takamaiman alamomi
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Tare da gajerun hanyoyin kuɗaɗen dijital
stonktop -s AAPL,BTC,ETH,SOL
```

### Zaɓuɓɓuka Masu Kama da `top`

```bash
# Saita tazarar sabuntawa (kamar top -d)
stonktop -s AAPL,GOOGL -d 10

# Gudanar da zagaye N sannan a fita (kamar top -n)
stonktop -s AAPL -n 5

# Yanayin rukuni domin rubutun sarrafawa (kamar top -b)
stonktop -s AAPL,GOOGL -b

# Yanayin tsaro - kashe umarnin mu'amala
stonktop -s AAPL -S
```

### Zaɓuɓɓukan Jeri

```bash
# Jera bisa farashi (daga sama zuwa ƙasa)
stonktop -s AAPL,GOOGL -o price

# Jera bisa alama (daga ƙasa zuwa sama)
stonktop -s AAPL,GOOGL -o symbol -r

# Filayen jeri da ake da su: symbol, name, price, change, change-percent, volume, market-cap
```

### Fayil ɗin Saiti

```bash
# Yi amfani da takamaiman fayil ɗin saiti
stonktop -c ~/.config/stonktop/custom.toml

# Ko ajiye fayil ɗin saiti a wurin da aka tanada:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Zaɓuɓɓukan Layin Umarni

| Zaɓi | Gajere | Bayani |
|------|--------|--------|
| `--symbols` | `-s` | Jerin alamomin da za a sa wa ido da aka raba da waƙafi |
| `--delay` | `-d` | Tazarar sabuntawa a cikin daƙiƙu (tsoho: 5) |
| `--iterations` | `-n` | Adadin zagayen sabuntawa (0 = mara iyaka) |
| `--batch` | `-b` | Yanayin rukuni - fitar da bayanai ba tare da mu'amala ba |
| `--secure` | `-S` | Yanayin tsaro - kashe umarnin mu'amala |
| `--config` | `-c` | Hanyar zuwa fayil ɗin saiti |
| `--sort` | `-o` | Filin jeri na farko |
| `--reverse` | `-r` | Juya tsarin jeri |
| `--holdings` | `-H` | Nuna ɓangaren kadarori da kundin hannun jari |
| `--currency` | | Kuɗin da za a nuna farashi da shi (tsoho: USD) |
| `--timeout` | | Iyakar lokacin jiran API a daƙiƙu (tsoho: 10) |
| `--format` | | Tsarin fitarwa a yanayin rukuni: table, json, csv |
| `--color` | | Fitar da launi: auto, always, never |
| `--verbose` | `-v` | Cikakken bayanin fitarwa |
| `--init` | | Ƙirƙiri samfurin fayil ɗin saiti |
| `--force` | | Tilasta maye gurbin tsohon saiti (tare da `--init`) |
| `--lang` | | Harshen nuni (`STONKTOP_LANG`, tsoho: kai tsaye daga `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Nuna saƙon taimako |
| `--version` | `-V` | Nuna lambar siga |

## Harsunan da ake Goyon Baya

Stonktop yana gano harshen nuni ta atomatik daga `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, da `LANG`, ko kuma yana karɓar `--lang <LANG>`. Dukkan harsuna 45 da ake goyon baya sun ƙunshi cikakkiyar fassarar CLI da TUI (har da tsarin rubutu daga dama zuwa hagu don Larabci, Farisanci, Ibrananci, da Urdu) tare da takardun jagora a kowane harshe:

| Lamba | Harshe | README | Lamba | Harshe | README | Lamba | Harshe | README |
|-------|--------|--------|-------|--------|--------|-------|--------|--------|
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

## Gajerun Hanyoyin Maɓallan Madannai

| Maɓalli | Aiki |
|---------|------|
| `q`, `Esc` | Fita |
| `h`, `?` | Nuna ko ɓoye taimako |
| `↑`, `k` | Matsa sama |
| `↓`, `j` | Matsa ƙasa |
| `g`, `Home` | Je saman jeri |
| `G`, `End` | Je ƙasan jeri |
| `PgUp` | Shafi na sama |
| `PgDn` | Shafi na ƙasa |
| `s` | Sauya filin jeri |
| `r` | Juya tsarin jeri |
| `1-7` | Jera bisa ginshiƙi |
| `Enter` | Nuna cikakken bayanin alamar da aka zaɓa |
| `/` | Bincika ko tace alamomi |
| `a` | Ƙara alama a jerin sa ido |
| `d` | Cire alamar da aka zaɓa |
| `S` | Nuna ko ɓoye gefen jadawalin sparkline |
| `e` | Fitar da jerin sa ido zuwa JSON |
| `H` | Sauya zuwa kallon kadarori |
| `f` | Nuna ko ɓoye muhimman bayanai (fundamentals) |
| `Space`, `R` | Tilasta sabuntawa |
| `Tab` | Zagaya tsakanin rukunin alamomi |

## Saiti

Ƙirƙiri fayil ɗin saiti na TOML:

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

# Kadarorin kundin hannun jari (na zaɓi)
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

# Rukunin alamomi
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Nunin kuɗaɗe masu yawa (na zaɓi)
[currency]
display = "EUR"
convert = true

# Sanarwar farashi (na zaɓi)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Gajerun hanyoyin kuɗaɗen dijital na musamman (na zaɓi)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Tsarin Alamomi

| Tsari | Bayani | Misali |
|-------|--------|--------|
| Hannun jari | Alamar kasuwa ta gama-gari | `AAPL`, `GOOGL` |
| Kuɗin dijital | Tare da ƙarshen USD | `BTC-USD`, `ETH-USD` |
| Gajeren sunan kuɗin dijital | Yana faɗaɗa zuwa `-USD` | `BTC.X` -> `BTC-USD` |
| Kuɗin dijital ta atomatik | Shahararrun kuɗaɗen dijital | `BTC` -> `BTC-USD` |

## Tushen Bayanai

Stonktop yana amfani da Yahoo Finance API don ɗauko farashi kai tsaye. Ba a buƙatar maɓallin API (API key).

## Haɓakawa

### Ginawa

```bash
# Gina sigar gwaji (Debug)
cargo build

# Gina sigar saki (Release)
cargo build --release
```

### Gwaje-gwaje

```bash
# Gudanar da dukkan gwaje-gwaje
cargo test

# Gudanar da gwaje-gwaje tare da nuna sakamako
cargo test -- --nocapture
```

### Binciken Lamba (Lint)

```bash
# Bincika matsalolin tsarin lamba
cargo clippy --all-targets --all-features

# Aiwatar da gyare-gyare ta atomatik
cargo clippy --fix --allow-dirty
```

### Tsara Rubutun Lamba

```bash
# Duba tsarin rubutun lamba
cargo fmt -- --check

# Aiwatar da tsarin rubutun lamba
cargo fmt
```

## Gudunmawa

Ana maraba da gudunmawa! Kuna iya gabatar da Pull Request a kowane lokaci.

1. Yi Fork na ma'ajiyar lamba
2. Ƙirƙiri reshen sabon fasali (`git checkout -b feature/amazing-feature`)
3. Ajiye sauye-sauyenka (`git commit -m 'Add amazing feature'`)
4. Tura reshen zuwa ma'ajiya (`git push origin feature/amazing-feature`)
5. Buɗe Pull Request

## Lasisi

Wannan aikin yana ƙarƙashin lasisin MIT — duba fayil ɗin [LICENSE](../../../LICENSE) don ƙarin bayani.

## Godiya

- An samo wahayi daga [ticker](https://github.com/achannarasappa/ticker)
- An gina shi da [ratatui](https://github.com/ratatui-org/ratatui)
- Bayanai daga [Yahoo Finance](https://finance.yahoo.com/)
