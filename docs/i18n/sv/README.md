# Stonktop

> [English](../../../README.md)

Ett `top`-liknande terminalgränssnitt för realtidsövervakning av aktie- och kryptovalutakurser.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Funktioner

- **Kursövervakning i realtid**: aktuella kurser för aktier och kryptovalutor
- **`top`-liknande gränssnitt**: välbekanta tangentbordskontroller som liknar `top`/`htop`
- **Sparkline-diagram**: integrerad visualisering av pristrender i ett sidofält
- **Portföljspårning**: övervaka dina innehav med beräkning av vinst/förlust (P/L)
- **Sök och filtrering**: skiftlägesokänslig sökning bland symboler och namn
- **Prisvarningar**: skrivbordsaviseringar när kurser passerar konfigurerade tröskelvärden
- **Export och import**: spara och läs in bevakningslistor som JSON eller CSV
- **Flera valutor**: visa priser i valfri valuta med aktuella växelkurser
- **Flera sorteringsalternativ**: sortera efter symbol, pris, förändring, volym och börsvärde
- **Batchläge**: icke-interaktiv utdata för skriptning (som `top -b`)
- **TOML-konfiguration**: lättredigerade konfigurationsfiler
- **Färgkodning**: grönt för uppgångar, rött för nedgångar
- **Kryptogenomvägar**: använd `BTC.X` eller enbart `BTC` för `BTC-USD`

## Skärmbilder

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

## Förutsättningar

- **Rust**: version 1.88 eller senare (MSRV)
- **Cargo**: ingår i Rust-installationen

## Installation

### Från crates.io (rekommenderas)

```bash
cargo install stonktop
```

### Med cargo-binstall

Om du har [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installerat kan du installera förkompilerade binärer:

```bash
cargo binstall stonktop
```

### Förkompilerade binärer

Ladda ned förkompilerade binärer från sidan [Releases](https://github.com/somethingwithproof/stonktop/releases).

Tillgängliga plattformar:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Från källkod

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Användning

### Grundläggande användning

```bash
# Övervaka specifika symboler
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Med kryptogenomvägar
stonktop -s AAPL,BTC,ETH,SOL
```

### `top`-liknande alternativ

```bash
# Ange uppdateringsintervall (som top -d)
stonktop -s AAPL,GOOGL -d 10

# Kör N iterationer och avsluta sedan (som top -n)
stonktop -s AAPL -n 5

# Batchläge för skript (som top -b)
stonktop -s AAPL,GOOGL -b

# Säkert läge - inaktivera interaktiva kommandon
stonktop -s AAPL -S
```

### Sorteringsalternativ

```bash
# Sortera efter pris (fallande)
stonktop -s AAPL,GOOGL -o price

# Sortera efter symbol (stigande)
stonktop -s AAPL,GOOGL -o symbol -r

# Tillgängliga sorteringsfält: symbol, name, price, change, change-percent, volume, market-cap
```

### Konfigurationsfil

```bash
# Använd en specifik konfigurationsfil
stonktop -c ~/.config/stonktop/custom.toml

# Eller placera konfigurationen på standardplatsen:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Kommandoradsflaggor

| Flagga | Kort | Beskrivning |
|--------|------|-------------|
| `--symbols` | `-s` | Kommaseparerad lista med symboler att övervaka |
| `--delay` | `-d` | Uppdateringsintervall i sekunder (standard: 5) |
| `--iterations` | `-n` | Antal iterationer (0 = oändligt) |
| `--batch` | `-b` | Batchläge - icke-interaktiv utdata |
| `--secure` | `-S` | Säkert läge - inaktivera interaktiva kommandon |
| `--config` | `-c` | Sökväg till konfigurationsfil |
| `--sort` | `-o` | Initialt sorteringsfält |
| `--reverse` | `-r` | Omvänd sorteringsordning |
| `--holdings` | `-H` | Visa innehavs-/portföljvy |
| `--currency` | | Visningsvaluta (standard: USD) |
| `--timeout` | | API-tidsgräns i sekunder (standard: 10) |
| `--format` | | Utdataformat för batchläge: table, json, csv |
| `--color` | | Färgutdata: auto, always, never |
| `--verbose` | `-v` | Utförlig utdata |
| `--init` | | Generera en exempelkonfigurationsfil |
| `--force` | | Tvinga överskrivning av befintlig konfiguration (tillsammans med `--init`) |
| `--lang` | | Visningsspråk (`STONKTOP_LANG`, standard: automatiskt från `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Visa hjälptext |
| `--version` | `-V` | Visa version |

## Språk som stöds

Stonktop identifierar visningsspråket från `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` och `LANG`, eller tar emot `--lang <LANG>`. Alla 45 lokaler som stöds omfattar fullständiga CLI- och TUI-översättningar (inklusive höger-till-vänster-justering för arabiska, persiska, hebreiska och urdu) samt lokaliserad dokumentation:

| Kod | Språk | README | Kod | Språk | README | Kod | Språk | README |
|-----|-------|--------|-----|-------|--------|-----|-------|--------|
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

## Tangentbordsgenvägar

| Tangent | Åtgärd |
|---------|--------|
| `q`, `Esc` | Avsluta |
| `h`, `?` | Visa/dölj hjälp |
| `↑`, `k` | Flytta upp |
| `↓`, `j` | Flytta ned |
| `g`, `Home` | Gå överst |
| `G`, `End` | Gå nederst |
| `PgUp` | Sida upp |
| `PgDn` | Sida ned |
| `s` | Växla sorteringsfält |
| `r` | Vänd sorteringsordning |
| `1-7` | Sortera efter kolumn |
| `Enter`, `d` | Visa detaljdialog för vald symbol |
| `/` | Sök/filtrera symboler |
| `a` | Lägg till en symbol i bevakningslistan |
| `x` | Ta bort vald symbol |
| `S` | Visa/dölj sidofältet för sparkline-diagram |
| `e` | Exportera bevakningslista till JSON |
| `H` | Växla innehavsvy |
| `f` | Visa/dölj nyckeltal |
| `Space`, `R` | Tvinga uppdatering |
| `Tab` | Växla symbolgrupper |

## Konfiguration

Skapa en TOML-konfigurationsfil:

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

# Portföljinnehav (valfritt)
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

# Symbolgrupper
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Visning i flera valutor (valfritt)
[currency]
display = "EUR"
convert = true

# Prisvarningar (valfritt)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Anpassade kryptogenomvägar (valfritt)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Symbolformat

| Format | Beskrivning | Exempel |
|--------|-------------|---------|
| Aktie | Standardticker | `AAPL`, `GOOGL` |
| Krypto | Med USD-suffix | `BTC-USD`, `ETH-USD` |
| Kryptoförkortning | Expanderas till `-USD` | `BTC.X` -> `BTC-USD` |
| Automatisk krypto | Vanliga kryptovalutor | `BTC` -> `BTC-USD` |

## Datakälla

Stonktop använder Yahoo Finance API för att hämta realtidskurser. Ingen API-nyckel krävs.

## Utveckling

### Bygg

```bash
# Felsökningsbygge
cargo build

# Release-bygge
cargo build --release
```

### Testa

```bash
# Kör alla tester
cargo test

# Kör tester med utdata
cargo test -- --nocapture
```

### Statisk analys

```bash
# Kontrollera lint-varningar
cargo clippy --all-targets --all-features

# Tillämpa automatiska åtgärder
cargo clippy --fix --allow-dirty
```

### Formatering

```bash
# Kontrollera formatering
cargo fmt -- --check

# Tillämpa formatering
cargo fmt
```

## Bidra

Bidrag är välkomna! Skicka gärna in en Pull Request.

1. Förgrena (forka) arkivet
2. Skapa din funktionsgren (`git checkout -b feature/amazing-feature`)
3. Bekräfta dina ändringar (`git commit -m 'Add amazing feature'`)
4. Skicka upp grenen (`git push origin feature/amazing-feature`)
5. Öppna en Pull Request

## Licens

Detta projekt är licensierat under MIT-licensen – se filen [LICENSE](../../../LICENSE) för mer information.

## Erkännanden

- Inspirerat av [ticker](https://github.com/achannarasappa/ticker)
- Byggt med [ratatui](https://github.com/ratatui-org/ratatui)
- Data från [Yahoo Finance](https://finance.yahoo.com/)
