# Stonktop

> [English](../../../README.md)

Een op `top` geïnspireerde terminal-gebruikersinterface voor het in realtime volgen van aandelen- en cryptokoersen.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Functies

- **Realtime koersbewaking**: Live koersen voor aandelen en cryptovaluta
- **`top`-achtige interface**: Vertrouwde toetsenbordbediening vergelijkbaar met `top`/`htop`
- **Sparkline-grafieken**: Inline visualisatie van koerstrends in een zijbalk
- **Portefeuillebeheer**: Volg uw posities inclusief winst- en verliesberekeningen (W/V)
- **Zoeken & filteren**: Hoofdletterongevoelig zoeken op symbolen en namen
- **Koersmeldingen**: Bureaubladmeldingen zodra koersen ingestelde drempelwaarden overschrijden
- **Exporteren/Importeren**: Volglijsten opslaan en laden als JSON of CSV
- **Meerdere valuta's**: Koersen weergeven in elke gewenste valuta met live wisselkoersen
- **Meerdere sorteeropties**: Sorteren op symbool, koers, wijziging, volume of marktkapitalisatie
- **Batchmodus**: Niet-interactieve uitvoer voor scripts (zoals `top -b`)
- **TOML-configuratie**: Eenvoudig te bewerken configuratiebestanden
- **Kleurcodering**: Groen voor winst, rood voor verlies
- **Crypto-snelkoppelingen**: Gebruik `BTC.X` of gewoon `BTC` voor `BTC-USD`

## Schermafbeeldingen

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

## Vereisten

- **Rust**: Versie 1.89 of hoger (MSRV)
- **Cargo**: Meegeleverd met de Rust-installatie

## Installatie

### Via crates.io (Aanbevolen)

```bash
cargo install stonktop
```

### Met cargo-binstall

Als [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) is geïnstalleerd, kunt u vooraf gecompileerde binaire bestanden installeren:

```bash
cargo binstall stonktop
```

### Vooraf gecompileerde binaire bestanden

Download vooraf gecompileerde binaire bestanden vanaf de pagina [Releases](https://github.com/somethingwithproof/stonktop/releases).

Beschikbare platformen:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Vanaf de broncode

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Gebruik

### Basisgebruik

```bash
# Specifieke symbolen volgen
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Met crypto-snelkoppelingen
stonktop -s AAPL,BTC,ETH,SOL
```

### `top`-achtige opties

```bash
# Vernieuwingsinterval instellen (zoals top -d)
stonktop -s AAPL,GOOGL -d 10

# N iteraties uitvoeren en daarna afsluiten (zoals top -n)
stonktop -s AAPL -n 5

# Batchmodus voor scripts (zoals top -b)
stonktop -s AAPL,GOOGL -b

# Beveiligde modus — interactieve opdrachten uitschakelen
stonktop -s AAPL -S
```

### Sorteeropties

```bash
# Sorteren op koers (aflopend)
stonktop -s AAPL,GOOGL -o price

# Sorteren op symbool (oplopend)
stonktop -s AAPL,GOOGL -o symbol -r

# Beschikbare sorteervelden: symbol, name, price, change, change-percent, volume, market-cap
```

### Configuratiebestand

```bash
# Een specifiek configuratiebestand gebruiken
stonktop -c ~/.config/stonktop/custom.toml

# Of plaats het configuratiebestand op de standaardlocatie:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opdrachtregelopties

| Optie | Kort | Beschrijving |
|-------|------|--------------|
| `--symbols` | `-s` | Kommagescheiden lijst met te volgen symbolen |
| `--delay` | `-d` | Vernieuwingsinterval in seconden (standaard: 5) |
| `--iterations` | `-n` | Aantal iteraties (0 = oneindig) |
| `--batch` | `-b` | Batchmodus — niet-interactieve uitvoer |
| `--secure` | `-S` | Beveiligde modus — interactieve opdrachten uitschakelen |
| `--config` | `-c` | Pad naar het configuratiebestand |
| `--sort` | `-o` | Initieel sorteerveld |
| `--reverse` | `-r` | Sorteervolgorde omkeren |
| `--holdings` | `-H` | Posities-/portefeuilleweergave tonen |
| `--currency` | | Weergavevaluta (standaard: USD) |
| `--timeout` | | API-time-out in seconden (standaard: 10) |
| `--format` | | Uitvoerformaat voor batchmodus: `table`, `json`, `csv` |
| `--color` | | Kleuruitvoer: `auto`, `always`, `never` |
| `--verbose` | `-v` | Uitgebreide uitvoer |
| `--init` | | Een voorbeeld-configuratiebestand genereren |
| `--force` | | Bestaande configuratie geforceerd overschrijven (met `--init`) |
| `--lang` | | Weergavetaal (`STONKTOP_LANG`, standaard: automatisch vanuit `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Helptekst tonen |
| `--version` | `-V` | Versie tonen |

## Ondersteunde talen

Stonktop detecteert de weergavetaal aan de hand van `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` en `LANG`, of accepteert `--lang <LANG>`. Alle 45 ondersteunde taalregio's bevatten volledige CLI- en TUI-vertalingen (inclusief rechts-naar-links-uitlijning voor Arabisch, Perzisch, Hebreeuws en Urdu) en gelokaliseerde documentatie:

| Code | Taal | README | Code | Taal | README | Code | Taal | README |
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

## Sneltoetsen

| Toets | Actie |
|-------|-------|
| `q`, `Esc` | Afsluiten |
| `h`, `?` | Help tonen/verbergen |
| `↑`, `k` | Omhoog verplaatsen |
| `↓`, `j` | Omlaag verplaatsen |
| `g`, `Home` | Naar boven gaan |
| `G`, `End` | Naar beneden gaan |
| `PgUp` | Pagina omhoog |
| `PgDn` | Pagina omlaag |
| `s` | Wisselen van sorteerveld |
| `r` | Sorteervolgorde omkeren |
| `1-7` | Sorteren op kolom |
| `Enter` | Detailvenster tonen voor het geselecteerde symbool |
| `/` | Symbolen zoeken/filteren |
| `a` | Een symbool toevoegen aan de volglijst |
| `d` | Geselecteerd symbool verwijderen |
| `S` | Zijbalk met sparkline-grafieken tonen/verbergen |
| `e` | Volglijst exporteren naar JSON |
| `H` | Positiesweergave tonen/verbergen |
| `f` | Fundamentele gegevens tonen/verbergen |
| `Space`, `R` | Vernieuwen forceren |
| `Tab` | Wisselen tussen symboolgroepen |

## Configuratie

Maak een TOML-configuratiebestand aan:

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

# Portefeuilleposities (optioneel)
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

# Symboolgroepen
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Weergave in meerdere valuta's (optioneel)
[currency]
display = "EUR"
convert = true

# Koersmeldingen (optioneel)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Aangepaste crypto-snelkoppelingen (optioneel)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Symboolformaten

| Formaat | Beschrijving | Voorbeeld |
|---------|--------------|-----------|
| Aandeel | Standaard tickersymbool | `AAPL`, `GOOGL` |
| Crypto | Met USD-achtervoegsel | `BTC-USD`, `ETH-USD` |
| Crypto-verkorting | Wordt uitgebreid naar `-USD` | `BTC.X` -> `BTC-USD` |
| Crypto automatisch | Gangbare cryptovaluta | `BTC` -> `BTC-USD` |

## Gegevensbron

Stonktop gebruikt de Yahoo Finance API om realtime koersen op te halen. Een API-sleutel is niet vereist.

## Ontwikkeling

### Compileren

```bash
# Debug-build
cargo build

# Release-build
cargo build --release
```

### Testen

```bash
# Alle tests uitvoeren
cargo test

# Tests uitvoeren met volledige uitvoer
cargo test -- --nocapture
```

### Linting

```bash
# Controleren op lint-meldingen
cargo clippy --all-targets --all-features

# Automatische correcties toepassen
cargo clippy --fix --allow-dirty
```

### Opmaak

```bash
# Code-opmaak controleren
cargo fmt -- --check

# Code-opmaak toepassen
cargo fmt
```

## Bijdragen

Bijdragen zijn van harte welkom! Dien gerust een Pull Request in.

1. Fork de repository
2. Maak uw feature-branch aan (`git checkout -b feature/amazing-feature`)
3. Commit uw wijzigingen (`git commit -m 'Add amazing feature'`)
4. Push de branch (`git push origin feature/amazing-feature`)
5. Open een Pull Request

## Licentie

Dit project valt onder de MIT-licentie — zie het bestand [LICENSE](../../../LICENSE) voor details.

## Dankbetuigingen

- Geïnspireerd door [ticker](https://github.com/achannarasappa/ticker)
- Gebouwd met [ratatui](https://github.com/ratatui-org/ratatui)
- Gegevens afkomstig van [Yahoo Finance](https://finance.yahoo.com/)
