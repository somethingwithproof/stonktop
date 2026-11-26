# Stonktop

> [English](../../../README.md)

'n `top`-agtige terminaal-gebruikerskoppelvlak (TUI) om aandele- en kriptogeldeenheidpryse intyds te monitor.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Kenmerke

- **Intydse Prysmonitering**: Regstreekse kwotasies vir aandele en kriptogeldeenhede
- **`top`-agtige Koppelvlak**: Bekende sleutelbordbeheer soortgelyk aan `top`/`htop`
- **Vonklyngrafieke (Sparkline)**: Inlyn-visualisering van prystendense in 'n sybalk
- **Portefeuljenasporing**: Monitor jou beleggings met wins-/verliesberekeninge (P/L)
- **Soek en Filter**: Hoofletter-onsensitiewe soektog oor simbole en name
- **Pryswaarskuwings**: Werkskermkennisgewings wanneer pryse gekonfigureerde drempels oorsteek
- **Uitvoer/Invoer**: Stoor en laai dophoulyste as JSON of CSV
- **Meervoudige Geldeenhede**: Vertoon pryse in enige geldeenheid met regstreekse wisselkoerse
- **Meervoudige Sorteeropsies**: Sorteer volgens simbool, prys, verandering, volume of markkapitalisasie
- **Bondelmodus (Batch Mode)**: Nie-interaktiewe afvoer vir skripte (soos `top -b`)
- **TOML-konfigurasie**: Konfigurasielêers wat maklik is om te redigeer
- **Kleurkodering**: Groen vir stygings, rooi vir dalings
- **Kripto-kortpaaie**: Gebruik `BTC.X` of bloot `BTC` vir `BTC-USD`

## Skermkiekies

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

## Voorvereistes

- **Rust**: Weergawe 1.89 of hoër (MSRV)
- **Cargo**: Ingesluit by die Rust-installasie

## Installasie

### Vanaf crates.io (Aanbeveel)

```bash
cargo install stonktop
```

### Met behulp van cargo-binstall

As jy [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) geïnstalleer het, kan jy voorafgeboude binêre lêers direk installeer:

```bash
cargo binstall stonktop
```

### Voorafgeboude Binêre Lêers

Laai voorafgeboude binêre lêers af vanaf die [Vrystellings (Releases)](https://github.com/somethingwithproof/stonktop/releases)-bladsy.

Beskikbare platforms:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Bou vanaf Bronkode

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Gebruik

### Basiese Gebruik

```bash
# Monitor spesifieke simbole
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Met kripto-kortpaaie
stonktop -s AAPL,BTC,ETH,SOL
```

### top-agtige Opsies

```bash
# Stel verversingsvertraging in (soos top -d)
stonktop -s AAPL,GOOGL -d 10

# Voer N iterasies uit en sluit dan af (soos top -n)
stonktop -s AAPL -n 5

# Bondelmodus vir skripte (soos top -b)
stonktop -s AAPL,GOOGL -b

# Veilige modus - deaktiveer interaktiewe opdragte
stonktop -s AAPL -S
```

### Sorteeropsies

```bash
# Sorteer volgens prys (dalend)
stonktop -s AAPL,GOOGL -o price

# Sorteer volgens simbool (stygend)
stonktop -s AAPL,GOOGL -o symbol -r

# Beskikbare sorteervelde: symbol, name, price, change, change-percent, volume, market-cap
```

### Konfigurasielêer

```bash
# Gebruik 'n spesifieke konfigurasielêer
stonktop -c ~/.config/stonktop/custom.toml

# Of plaas die konfigurasielêer op die verstekligging:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opdragreëlopsies

| Opsie | Kort | Beskrywing |
|-------|------|------------|
| `--symbols` | `-s` | Komma-geskeide lys van simbole om te monitor |
| `--delay` | `-d` | Verversingsvertraging in sekondes (verstek: 5) |
| `--iterations` | `-n` | Aantal iterasies (0 = oneindig) |
| `--batch` | `-b` | Bondelmodus - nie-interaktiewe afvoer |
| `--secure` | `-S` | Veilige modus - deaktiveer interaktiewe opdragte |
| `--config` | `-c` | Pad na konfigurasielêer |
| `--sort` | `-o` | Aanvanklike sorteerveld |
| `--reverse` | `-r` | Keer sorteervolgorde om |
| `--holdings` | `-H` | Vertoon besit-/portefeulje-aansig |
| `--currency` | | Vertoongeldeenheid (verstek: USD) |
| `--timeout` | | API-tydverstryking in sekondes (verstek: 10) |
| `--format` | | Afvoerformaat vir bondelmodus: table, json, csv |
| `--color` | | Kleurafvoer: auto, always, never |
| `--verbose` | `-v` | Uitvoerige afvoer |
| `--init` | | Genereer 'n voorbeeldkonfigurasielêer |
| `--force` | | Dwing oorskrywing van bestaande konfigurasie (saam met `--init`) |
| `--lang` | | Vertoontaal (`STONKTOP_LANG`, verstek: outomaties vanaf `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Vertoon hulpboodskap |
| `--version` | `-V` | Vertoon weergawe |

## Ondersteunde Tale

Stonktop bespeur die vertoontaal vanaf `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` en `LANG`, of aanvaar `--lang <LANG>`. Al 45 ondersteunde lokale sluit volledige CLI- en TUI-vertalings in (insluitend regs-na-links-belyning vir Arabies, Persies, Hebreeus en Oerdoe) asook gelokaliseerde dokumentasie:

| Kode | Taal | README | Kode | Taal | README | Kode | Taal | README |
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

## Sleutelbordkortpaaie

| Sleutel | Aksie |
|---------|-------|
| `q`, `Esc` | Sluit af |
| `h`, `?` | Wissel hulp |
| `↑`, `k` | Beweeg op |
| `↓`, `j` | Beweeg af |
| `g`, `Home` | Gaan na bo |
| `G`, `End` | Gaan na onder |
| `PgUp` | Bladsy op |
| `PgDn` | Bladsy af |
| `s` | Wissel deur sorteervelde |
| `r` | Keer sorteervolgorde om |
| `1-7` | Sorteer volgens kolom |
| `Enter` | Vertoon detail-opwipvenster vir gekose simbool |
| `/` | Soek/filter simbole |
| `a` | Voeg 'n simbool by dophoulys |
| `d` | Verwyder gekose simbool |
| `S` | Wissel vonklyngrafieke-sybalk |
| `e` | Voer dophoulys uit na JSON |
| `H` | Wissel besit-aansig |
| `f` | Wissel fundamentele gegewens |
| `Space`, `R` | Dwing verversing |
| `Tab` | Wissel deur simboolgroepe |

## Konfigurasie

Skep 'n TOML-konfigurasielêer:

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

# Portefeuljebesit (opsioneel)
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

# Simboolgroepe
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Meervoudige geldeenheid-vertoning (opsioneel)
[currency]
display = "EUR"
convert = true

# Pryswaarskuwings (opsioneel)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Gepasmaakte kripto-kortpaaie (opsioneel)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Simboolformate

| Formaat | Beskrywing | Voorbeeld |
|---------|------------|-----------|
| Aandeel | Standaard-aandeelsimbool | `AAPL`, `GOOGL` |
| Kripto | Met USD-agtervoegsel | `BTC-USD`, `ETH-USD` |
| Kripto-kortvorm | Brei uit na -USD | `BTC.X` -> `BTC-USD` |
| Kripto-outo | Algemene kriptogeldeenhede | `BTC` -> `BTC-USD` |

## Databron

Stonktop gebruik die Yahoo Finance API om intydse kwotasies af te haal. Geen API-sleutel word vereis nie.

## Ontwikkeling

### Bou

```bash
# Ontfoutbou (debug)
cargo build

# Vrystellingbou (release)
cargo build --release
```

### Toetsing

```bash
# Voer alle toetse uit
cargo test

# Voer toetse met afvoer uit
cargo test -- --nocapture
```

### Statiese Analise (Lint)

```bash
# Kontroleer vir lint-probleme
cargo clippy --all-targets --all-features

# Pas outomatiese regstellings toe
cargo clippy --fix --allow-dirty
```

### Formatering

```bash
# Kontroleer formatering
cargo fmt -- --check

# Pas formatering toe
cargo fmt
```

## Bydraes

Bydraes is baie welkom! Dien gerus 'n Pull Request in.

1. Maak 'n vurk (Fork) van die bewaarplek
2. Skep jou kenmerktak (`git checkout -b feature/amazing-feature`)
3. Bevestig (commit) jou veranderinge (`git commit -m 'Add amazing feature'`)
4. Stoot die tak op (`git push origin feature/amazing-feature`)
5. Maak 'n Pull Request oop

## Lisensie

Hierdie projek is gelisensieer onder die MIT-lisensie — sien die [LICENSE](../../../LICENSE)-lêer vir besonderhede.

## Erkennings

- Geïnspireer deur [ticker](https://github.com/achannarasappa/ticker)
- Gebou met [ratatui](https://github.com/ratatui-org/ratatui)
- Data vanaf [Yahoo Finance](https://finance.yahoo.com/)
