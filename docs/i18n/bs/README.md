# Stonktop

> [English](../../../README.md)

Terminalni interfejs nalik alatu `top` za praćenje cijena dionica i kriptovaluta u stvarnom vremenu.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Mogućnosti

- **Praćenje cijena u stvarnom vremenu**: Kotacije uživo za dionice i kriptovalute
- **Interfejs u stilu alata `top`**: Poznate prečice na tastaturi slične alatima `top`/`htop`
- **Sparkline grafikoni**: Prikaz trenda cijena u bočnoj traci
- **Praćenje portfelja**: Pratite svoje pozicije s izračunom dobiti i gubitka (P/L)
- **Pretraga i filtriranje**: Pretraga simbola i naziva bez obzira na veličinu slova
- **Upozorenja o cijenama**: Obavještenja na radnoj površini kada cijene pređu podešene pragove
- **Izvoz/Uvoz**: Sačuvajte i učitajte liste praćenja u JSON ili CSV formatu
- **Više valuta**: Prikaz cijena u bilo kojoj valuti uz aktuelne kurseve konverzije
- **Više opcija sortiranja**: Sortiranje po simbolu, cijeni, promjeni, obimu ili tržišnoj kapitalizaciji
- **Paketni način rada**: Neinteraktivni izlaz za skripte (kao `top -b`)
- **TOML postavke**: Konfiguracijske datoteke jednostavne za uređivanje
- **Označavanje bojama**: Zelena za rast, crvena za pad
- **Kripto prečice**: Koristite `BTC.X` ili samo `BTC` za `BTC-USD`

## Snimci ekrana

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

## Preduslovi

- **Rust**: verzija 1.89 ili novija (MSRV)
- **Cargo**: uključen uz instalaciju Rusta

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

## Korištenje

### Osnovno korištenje

```bash
# Pratite određene simbole
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# S kripto prečicama
stonktop -s AAPL,BTC,ETH,SOL
```

### Opcije nalik alatu top

```bash
# Postavite interval osvježavanja (kao top -d)
stonktop -s AAPL,GOOGL -d 10

# Pokrenite N iteracija i izađite (kao top -n)
stonktop -s AAPL -n 5

# Paketni način rada za skripte (kao top -b)
stonktop -s AAPL,GOOGL -b

# Sigurni način rada - onemogućava interaktivne naredbe
stonktop -s AAPL -S
```

### Opcije sortiranja

```bash
# Sortiraj po cijeni (opadajuće)
stonktop -s AAPL,GOOGL -o price

# Sortiraj po simbolu (rastuće)
stonktop -s AAPL,GOOGL -o symbol -r

# Dostupna polja sortiranja: symbol, name, price, change, change-percent, volume, market-cap
```

### Konfiguracijska datoteka

```bash
# Koristite određenu konfiguracijsku datoteku
stonktop -c ~/.config/stonktop/custom.toml

# Ili postavite konfiguraciju na zadanu lokaciju:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opcije komandne linije

| Opcija | Kratko | Opis |
|--------|-------|-------------|
| `--symbols` | `-s` | Lista simbola za praćenje odvojena zarezom |
| `--delay` | `-d` | Interval osvježavanja u sekundama (zadano: 5) |
| `--iterations` | `-n` | Broj iteracija (0 = beskonačno) |
| `--batch` | `-b` | Paketni način rada - neinteraktivni izlaz |
| `--secure` | `-S` | Sigurni način rada - onemogućava interaktivne naredbe |
| `--config` | `-c` | Putanja do konfiguracijske datoteke |
| `--sort` | `-o` | Početno polje sortiranja |
| `--reverse` | `-r` | Obrnuti redoslijed sortiranja |
| `--holdings` | `-H` | Prikaži pregled portfelja i pozicija |
| `--currency` | | Valuta prikaza (zadano: USD) |
| `--timeout` | | Vremensko ograničenje API zahtjeva u sekundama (zadano: 10) |
| `--format` | | Format izlaza za paketni način rada: table, json, csv |
| `--color` | | Izlaz u boji: auto, always, never |
| `--verbose` | `-v` | Detaljan izlaz |
| `--init` | | Generiši primjer konfiguracijske datoteke |
| `--force` | | Prisilno prepiši postojeće postavke (uz `--init`) |
| `--lang` | | Jezik prikaza (`STONKTOP_LANG`, zadano: automatski iz `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Prikaži poruku pomoći |
| `--version` | `-V` | Prikaži verziju |

## Podržani jezici

Stonktop prepoznaje jezik prikaza iz varijabli `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` i `LANG`, ili prihvata opciju `--lang <LANG>`. Svih 48 podržanih lokaliteta uključuje potpune prijevode CLI i TUI interfejsa (uključujući poravnanje zdesna nalijevo za arapski, perzijski, hebrejski i urdu) i lokalizovanu dokumentaciju:

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
| `ko` | 한국어 | [한국어](../ko/README.md) | `so-SO` | Soomaali | [Soomaali](../so-SO/README.md) | `bs` | Bosanski | **Bosanski** |
| `zh-CN` | 简体中文 | [简体中文](../zh-CN/README.md) | `sw-KE` | Kiswahili | [Kiswahili](../sw-KE/README.md) | `hr` | Hrvatski | [Hrvatski](../hr/README.md) |
| `zh-TW` | 繁體中文 | [繁體中文](../zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](../xh-ZA/README.md) | `sr` | Српски | [Српски](../sr/README.md) |
| `ar` | العربية | [العربية](../ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](../yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](../hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](../zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](../pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](../tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](../sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](../vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](../id/README.md) | | | | | | |

## Prečice na tastaturi

| Tipka | Radnja |
|-----|--------|
| `q`, `Esc` | Izlaz |
| `h`, `?` | Prikaži/sakrij pomoć |
| `↑`, `k` | Pomjeri gore |
| `↓`, `j` | Pomjeri dolje |
| `g`, `Home` | Idi na vrh |
| `G`, `End` | Idi na dno |
| `PgUp` | Stranica gore |
| `PgDn` | Stranica dolje |
| `s` | Promijeni polje sortiranja |
| `r` | Obrni redoslijed sortiranja |
| `1-7` | Sortiraj po koloni |
| `Enter` | Prikaži detalje za odabrani simbol |
| `/` | Pretraži/filtriraj simbole |
| `a` | Dodaj simbol na listu praćenja |
| `d` | Ukloni odabrani simbol |
| `S` | Prikaži/sakrij bočnu traku sa sparkline graficima |
| `e` | Izvezi listu praćenja u JSON |
| `H` | Prikaži/sakrij pregled pozicija |
| `f` | Prikaži/sakrij osnovne podatke |
| `Space`, `R` | Prisilno osvježi |
| `Tab` | Promijeni grupu simbola |

## Postavke

Kreirajte TOML konfiguracijsku datoteku:

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

# Pozicije u portfelju (opcionalno)
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

# Grupe simbola
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Prikaz u više valuta (opcionalno)
[currency]
display = "EUR"
convert = true

# Upozorenja o cijenama (opcionalno)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Prilagođene kripto prečice (opcionalno)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formati simbola

| Format | Opis | Primjer |
|--------|-------------|---------|
| Dionica | Standardna oznaka (ticker) | `AAPL`, `GOOGL` |
| Kripto | Sa sufiksom USD | `BTC-USD`, `ETH-USD` |
| Kripto skraćenica | Proširuje se u -USD | `BTC.X` -> `BTC-USD` |
| Automatski kripto | Česte kriptovalute | `BTC` -> `BTC-USD` |

## Izvor podataka

Stonktop koristi Yahoo Finance API za preuzimanje kotacija u stvarnom vremenu. API ključ nije potreban.

## Razvoj

### Izgradnja

```bash
# Debug izgradnja
cargo build

# Release izgradnja
cargo build --release
```

### Testiranje

```bash
# Pokrenite sve testove
cargo test

# Pokrenite testove s prikazom izlaza
cargo test -- --nocapture
```

### Provjera koda (Lint)

```bash
# Provjerite probleme statičke analize
cargo clippy --all-targets --all-features

# Primijenite automatske ispravke
cargo clippy --fix --allow-dirty
```

### Formatiranje

```bash
# Provjerite formatiranje
cargo fmt -- --check

# Primijenite formatiranje
cargo fmt
```

## Doprinosi

Doprinosi su dobrodošli! Slobodno pošaljite Pull Request.

1. Napravite fork repozitorija
2. Kreirajte svoju granu za funkcionalnost (`git checkout -b feature/amazing-feature`)
3. Pripremite izmijenjene datoteke (`git add <datoteke>`)
4. Sačuvajte izmjene (`git commit -m 'Add amazing feature'`)
5. Pošaljite granu na server (`git push origin feature/amazing-feature`)
6. Otvorite Pull Request

## Licenca

Ovaj projekat je licenciran pod MIT licencom - za detalje pogledajte datoteku [LICENSE](../../../LICENSE).

## Zahvale

- Inspirisano alatom [ticker](https://github.com/achannarasappa/ticker)
- Izgrađeno pomoću [ratatui](https://github.com/ratatui-org/ratatui)
- Podaci s [Yahoo Finance](https://finance.yahoo.com/)
