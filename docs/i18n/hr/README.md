# Stonktop

> [English](../../../README.md)

Terminalsko sučelje nalik alatu `top` za praćenje cijena dionica i kriptovaluta u stvarnom vremenu.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Značajke

- **Praćenje cijena u stvarnom vremenu**: Trenutačne kotacije dionica i kriptovaluta
- **Sučelje u stilu alata `top`**: Poznate tipkovničke naredbe slične alatima `top`/`htop`
- **Linijski grafikoni (Sparkline)**: Prikaz kretanja cijena u bočnoj traci
- **Praćenje portfelja**: Pratite svoje udjele uz izračun dobiti i gubitka (P/L)
- **Pretraživanje i filtriranje**: Pretraživanje simbola i naziva neovisno o veličini slova
- **Upozorenja o cijenama**: Obavijesti na radnoj površini kada cijene prijeđu zadane pragove
- **Izvoz/Uvoz**: Pohranite i učitajte popise za praćenje u JSON ili CSV formatu
- **Više valuta**: Prikaz cijena u bilo kojoj valuti uz ažurne tečajeve pretvorbe
- **Više mogućnosti razvrstavanja**: Razvrstavanje po simbolu, cijeni, promjeni, volumenu ili tržišnoj kapitalizaciji
- **Skupni način rada**: Neinteraktivni ispis za skripte (poput `top -b`)
- **TOML postavke**: Datoteke s postavkama jednostavne za uređivanje
- **Označavanje bojama**: Zelena za dobitke, crvena za gubitke
- **Kripto prečaci**: Koristite `BTC.X` ili samo `BTC` za `BTC-USD`

## Snimke zaslona

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

## Preduvjeti

- **Rust**: inačica 1.89 ili novija (MSRV)
- **Cargo**: uključen u instalaciju jezika Rust

## Instalacija

### Putem crates.io (preporučeno)

```bash
cargo install stonktop
```

### Pomoću cargo-binstall

Ako imate instaliran [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), možete instalirati unaprijed izgrađene binarne datoteke:

```bash
cargo binstall stonktop
```

### Unaprijed izgrađene binarne datoteke

Preuzmite unaprijed izgrađene binarne datoteke sa stranice [Izdanja](https://github.com/somethingwithproof/stonktop/releases).

Dostupne platforme:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Iz izvornog koda

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Upotreba

### Osnovna upotreba

```bash
# Watch specific symbols
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# With crypto shortcuts
stonktop -s AAPL,BTC,ETH,SOL
```

### Opcije slične alatu top

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

### Opcije razvrstavanja

```bash
# Sort by price (descending)
stonktop -s AAPL,GOOGL -o price

# Sort by symbol (ascending)
stonktop -s AAPL,GOOGL -o symbol -r

# Available sort fields: symbol, name, price, change, change-percent, volume, market-cap
```

### Datoteka s postavkama

```bash
# Use specific config file
stonktop -c ~/.config/stonktop/custom.toml

# Or place config at default location:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opcije naredbenog retka

| Opcija | Kratko | Opis |
|--------|-------|-------------|
| `--symbols` | `-s` | Popis simbola za praćenje odvojen zarezom |
| `--delay` | `-d` | Razmak osvježavanja u sekundama (zadano: 5) |
| `--iterations` | `-n` | Broj ponavljanja (0 = beskonačno) |
| `--batch` | `-b` | Skupni način rada - neinteraktivni ispis |
| `--secure` | `-S` | Sigurni način rada - onemogućuje interaktivne naredbe |
| `--config` | `-c` | Putanja do datoteke s postavkama |
| `--sort` | `-o` | Početno polje razvrstavanja |
| `--reverse` | `-r` | Obrnuti redoslijed razvrstavanja |
| `--holdings` | `-H` | Prikaži pregled portfelja i udjela |
| `--currency` | | Valuta prikaza (zadano: USD) |
| `--timeout` | | Vremensko ograničenje API zahtjeva u sekundama (zadano: 10) |
| `--format` | | Format ispisa za skupni način rada: table, json, csv |
| `--color` | | Ispis u boji: auto, always, never |
| `--verbose` | `-v` | Opširan ispis |
| `--init` | | Stvori oglednu datoteku s postavkama |
| `--force` | | Prisilno prepiši postojeće postavke (uz `--init`) |
| `--lang` | | Jezik prikaza (`STONKTOP_LANG`, zadano: automatski iz `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Prikaži poruku pomoći |
| `--version` | `-V` | Prikaži inačicu |

## Podržani jezici

Stonktop prepoznaje jezik prikaza iz varijabli `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` i `LANG`, ili prihvaća opciju `--lang <LANG>`. Svih 48 podržanih lokaliteta uključuje potpune prijevode CLI i TUI sučelja (uključujući poravnanje zdesna nalijevo za arapski, perzijski, hebrejski i urdu) te lokaliziranu dokumentaciju:

| Kod | Jezik | README | Kod | Jezik | README | Kod | Jezik | README |
|------|----------|--------|------|----------|--------|------|----------|--------|
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
| `ko` | 한국어 | [한국어](../ko/README.md) | `so-SO` | Soomaali | [Soomaali](../so-SO/README.md) | `bs` | Bosanski | [Bosanski](../bs/README.md) |
| `zh-CN` | 简体中文 | [简体中文](../zh-CN/README.md) | `sw-KE` | Kiswahili | [Kiswahili](../sw-KE/README.md) | `hr` | Hrvatski | **Hrvatski** |
| `zh-TW` | 繁體中文 | [繁體中文](../zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](../xh-ZA/README.md) | `sr` | Српски | [Српски](../sr/README.md) |
| `ar` | العربية | [العربية](../ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](../yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](../hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](../zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](../pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](../tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](../sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](../vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](../id/README.md) | | | | | | |

## Tipkovnički prečaci

| Tipka | Radnja |
|-----|--------|
| `q`, `Esc` | Izlaz |
| `h`, `?` | Prikaži/sakrij pomoć |
| `↑`, `k` | Pomakni gore |
| `↓`, `j` | Pomakni dolje |
| `g`, `Home` | Idi na vrh |
| `G`, `End` | Idi na dno |
| `PgUp` | Stranica gore |
| `PgDn` | Stranica dolje |
| `s` | Promijeni polje razvrstavanja |
| `r` | Obrni redoslijed razvrstavanja |
| `1-7` | Razvrstaj po stupcu |
| `Enter` | Prikaži pojedinosti za odabrani simbol |
| `/` | Pretraži/filtriraj simbole |
| `a` | Dodaj simbol na popis za praćenje |
| `d` | Ukloni odabrani simbol |
| `S` | Prikaži/sakrij bočnu traku s linijskim grafikonima |
| `e` | Izvezi popis za praćenje u JSON |
| `H` | Prikaži/sakrij pregled udjela |
| `f` | Prikaži/sakrij pokazatelje |
| `Space`, `R` | Prisilno osvježi |
| `Tab` | Promijeni skupinu simbola |

## Postavke

Izradite TOML datoteku s postavkama:

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

## Formati simbola

| Format | Opis | Primjer |
|--------|-------------|---------|
| Dionica | Standardna burzovna oznaka | `AAPL`, `GOOGL` |
| Kripto | Sa sufiksom USD | `BTC-USD`, `ETH-USD` |
| Kripto kratica | Proširuje se u -USD | `BTC.X` -> `BTC-USD` |
| Automatski kripto | Uobičajene kriptovalute | `BTC` -> `BTC-USD` |

## Izvor podataka

Stonktop koristi Yahoo Finance API za dohvat kotacija u stvarnom vremenu. API ključ nije potreban.

## Razvoj

### Izgradnja

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testiranje

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Provjera koda (Lint)

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Oblikovanje

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Doprinos projektu

Doprinosi su dobrodošli! Slobodno pošaljite Pull Request.

1. Napravite fork repozitorija
2. Stvorite svoju granu za značajku (`git checkout -b feature/amazing-feature`)
3. Pohranite promjene (`git commit -m 'Add amazing feature'`)
4. Pošaljite granu na poslužitelj (`git push origin feature/amazing-feature`)
5. Otvorite Pull Request

## Licenca

Ovaj je projekt licenciran pod MIT licencom - pojedinosti potražite u datoteci [LICENSE](../../../LICENSE).

## Zahvale

- Nadahnuto alatom [ticker](https://github.com/achannarasappa/ticker)
- Izrađeno pomoću [ratatui](https://github.com/ratatui-org/ratatui)
- Podatci s platforme [Yahoo Finance](https://finance.yahoo.com/)
