# Stonktop

> [English](../../../README.md)

Fampiharana an-tsoratra (terminal UI) mitovy amin'ny `top` ahafahana manara-maso mivantana ny vidin'ny petrabola sy ny vola nomerika (cryptomonnaie).

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Endri-javatra

- **Fanaraha-maso ny Vidiny Mivantana**: Tahan'ny petrabola sy ny vola nomerika amin'ny fotoana tena izy
- **Endrika Mitovy amin'ny `top`**: Fibaikoana amin'ny klavie mahazatra tahaka ny `top`/`htop`
- **Tabilao Kely Sparkline**: Fampisehoana eo an-tsisiny ny fivoaran'ny vidiny
- **Fanaraha-maso ny Pôrtafôlio**: Araho ny petrabola anananao miaraka amin'ny kajy tombony sy fatiantoka (P/L)
- **Fikarohana sy Fanivanana**: Fikarohana tsy manavaka sora-baventy sy sora-madinika amin'ny mari-pamantarana sy ny anarana
- **Fampandrenesana Vidiny**: Fampandrenesana eo amin'ny birao rehefa mihoatra ny fetra voafaritra ny vidiny
- **Fanondranana / Fanafarana**: Tehirizo sy ampidiro amin'ny endrika JSON na CSV ny lisitra arahinao
- **Vola Maro**: Asehoy amin'ny vola tianao ny vidiny miaraka amin'ny tahan'ny fifanakalozana mivantana
- **Fanasokajiana Maro**: Alaharo araka ny mari-pamantarana, vidiny, fiovana, habetsaky ny varotra, na sandan'ny tsena
- **Fomba Batch**: Famoahana tsy mila fifampiraharahana ho an'ny script (tahaka ny `top -b`)
- **Fanamboarana TOML**: Rakitra fanamboarana mora ovaina
- **Loko Famantarana**: Maitso ho an'ny tombony, mena ho an'ny fatiantoka
- **Hitsin-dalana ho an'ny Vola Nomerika**: Ampiasao ny `BTC.X` na `BTC` fotsiny ho an'ny `BTC-USD`

## Pikantsary

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

## Fepetra Ilaina Mialoha

- **Rust**: Dikan-teny 1.89 na ambony kokoa (MSRV)
- **Cargo**: Miaraka amin'ny fametrahana Rust

## Fametrahana

### Avy amin'ny crates.io (Atolotra)

```bash
cargo install stonktop
```

### Fampiasana cargo-binstall

Raha efa manana [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) ianao dia afaka mametraka mivantana ny rakitra efa voaomana:

```bash
cargo binstall stonktop
```

### Rakitra Efa Voaomana

Alaivo ny rakitra efa voaomana ao amin'ny pejy [Releases](https://github.com/somethingwithproof/stonktop/releases).

Rafitra misy:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Avy amin'ny Kaody Loharano

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Fampiasana

### Fampiasana Fototra

```bash
# Manara-maso mari-pamantarana manokana
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Miaraka amin'ny hitsin-dalana vola nomerika
stonktop -s AAPL,BTC,ETH,SOL
```

### Safidy Mitovy amin'ny `top`

```bash
# Mametraka ny elanelam-potoana fanavaozana (tahaka ny top -d)
stonktop -s AAPL,GOOGL -d 10

# Manatanteraka fihodinana N dia mivoaka (tahaka ny top -n)
stonktop -s AAPL -n 5

# Fomba batch ho an'ny script (tahaka ny top -b)
stonktop -s AAPL,GOOGL -b

# Fomba azo antoka - manakana ny baiko ifampiraharahana
stonktop -s AAPL -S
```

### Safidy Fanasokajiana

```bash
# Manasokajy araka ny vidiny (midina)
stonktop -s AAPL,GOOGL -o price

# Manasokajy araka ny mari-pamantarana (miakatra)
stonktop -s AAPL,GOOGL -o symbol -r

# Sahan'ny fanasokajiana misy: symbol, name, price, change, change-percent, volume, market-cap
```

### Rakitra Fanamboarana

```bash
# Mampiasa rakitra fanamboarana manokana
stonktop -c ~/.config/stonktop/custom.toml

# Na apetraho eo amin'ny toerana mahazatra ny rakitra:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Safidy amin'ny Andalana Baiko

| Safidy | Fohy | Famaritana |
|--------|------|------------|
| `--symbols` | `-s` | Lisitry ny mari-pamantarana arahina misaraka amin'ny faingo |
| `--delay` | `-d` | Elanelam-potoana fanavaozana amin'ny segondra (mahazatra: 5) |
| `--iterations` | `-n` | Isan'ny fihodinana (0 = tsy misy fetra) |
| `--batch` | `-b` | Fomba batch - famoahana tsy ifampiraharahana |
| `--secure` | `-S` | Fomba azo antoka - manakana ny baiko ifampiraharahana |
| `--config` | `-c` | Lalan'ny rakitra fanamboarana |
| `--sort` | `-o` | Sahan'ny fanasokajiana voalohany |
| `--reverse` | `-r` | Mamadika ny filaharan'ny fanasokajiana |
| `--holdings` | `-H` | Mampiseho ny fijery pôrtafôlio / fananana |
| `--currency` | | Vola aseho (mahazatra: USD) |
| `--timeout` | | Fetra fotoana fiandrasana API amin'ny segondra (mahazatra: 10) |
| `--format` | | Endrika famoahana amin'ny fomba batch: table, json, csv |
| `--color` | | Famoahana loko: auto, always, never |
| `--verbose` | `-v` | Famoahana antsipiriany |
| `--init` | | Mamorona rakitra fanamboarana ohatra |
| `--force` | | Manery hanoratra eo ambonin'ny rakitra efa misy (miaraka amin'ny `--init`) |
| `--lang` | | Fiteny aseho (`STONKTOP_LANG`, mahazatra: mandeha ho azy avy amin'ny `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Mampiseho hafatra fanampiana |
| `--version` | `-V` | Mampiseho dikan-teny |

## Fiteny Tohanana

Hitan'ny Stonktop ho azy ny fiteny aseho avy amin'ny `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, ary `LANG`, na mandray ny `--lang <LANG>`. Ny fiteny 45 rehetra tohanana dia ahitana fandikan-teny feno ho an'ny CLI sy TUI (anisan'izany ny filaharana ankavanana miankavia ho an'ny Arabo, Persiana, Hebreo, ary Ordo) ary antontan-taratasy voadika:

| Kaody | Fiteny | README | Kaody | Fiteny | README | Kaody | Fiteny | README |
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

## Hitsin-dalana amin'ny Klavie

| Bokotra | Asa |
|---------|-----|
| `q`, `Esc` | Mivoaka |
| `h`, `?` | Mampiseho na manafina fanampiana |
| `↑`, `k` | Miakatra |
| `↓`, `j` | Midina |
| `g`, `Home` | Mankany ambony indrindra |
| `G`, `End` | Mankany ambany indrindra |
| `PgUp` | Pejy teo aloha |
| `PgDn` | Pejy manaraka |
| `s` | Manova ny sahan'ny fanasokajiana |
| `r` | Mamadika ny filaharan'ny fanasokajiana |
| `1-7` | Manasokajy araka ny tsanganana |
| `Enter` | Mampiseho varavarankely misy antsipiriany momba ny mari-pamantarana voafidy |
| `/` | Mikaroka / manivana mari-pamantarana |
| `a` | Manampy mari-pamantarana amin'ny lisitra arahina |
| `d` | Manala ny mari-pamantarana voafidy |
| `S` | Mampiseho na manafina ny sisiny misy tabilao sparkline |
| `e` | Manondrana ny lisitra arahina ho JSON |
| `H` | Manova ny fijery pôrtafôlio |
| `f` | Mampiseho na manafina ny angona fototra (fundamentals) |
| `Space`, `R` | Manery fanavaozana |
| `Tab` | Mifindra eo amin'ireo vondrona mari-pamantarana |

## Fanamboarana

Mamorona rakitra fanamboarana TOML:

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

# Fananana ao amin'ny pôrtafôlio (tsy tsy maintsy)
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

# Vondrona mari-pamantarana
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Fampisehoana vola maro (tsy tsy maintsy)
[currency]
display = "EUR"
convert = true

# Fampandrenesana vidiny (tsy tsy maintsy)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Hitsin-dalana manokana ho an'ny vola nomerika (tsy tsy maintsy)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Endriky ny Mari-pamantarana

| Endrika | Famaritana | Ohatra |
|---------|------------|--------|
| Petrabola | Mari-pamantarana mahazatra | `AAPL`, `GOOGL` |
| Vola nomerika | Miaraka amin'ny tovana USD | `BTC-USD`, `ETH-USD` |
| Fanafohezana vola nomerika | Mivelatra ho `-USD` | `BTC.X` -> `BTC-USD` |
| Vola nomerika mandeha ho azy | Vola nomerika fampiasa matetika | `BTC` -> `BTC-USD` |

## Loharanon-data

Mampiasa ny Yahoo Finance API ny Stonktop mba hakana ny vidiny amin'ny fotoana tena izy. Tsy mila fanalahidy API (API key).

## Fampandrosoana

### Fananganana (Build)

```bash
# Fananganana debug
cargo build

# Fananganana release
cargo build --release
```

### Fitsapana

```bash
# Mandefa ny fitsapana rehetra
cargo test

# Mandefa fitsapana miaraka amin'ny famoahana
cargo test -- --nocapture
```

### Fanamarinana Kaody (Lint)

```bash
# Manamarina olana amin'ny lint
cargo clippy --all-targets --all-features

# Mampihatra fanitsiana mandeha ho azy
cargo clippy --fix --allow-dirty
```

### Fandrafetana Kaody

```bash
# Manamarina ny fandrafetana
cargo fmt -- --check

# Mampihatra ny fandrafetana
cargo fmt
```

## Fandraisana Anjara

Tongasoa ny fandraisana anjara! Aza misalasala mandefa Pull Request.

1. Manaova Fork ny tahiry
2. Mamorona sampana ho an'ny endri-javatra vaovao (`git checkout -b feature/amazing-feature`)
3. Raketo ny fanovana nataonao (`git commit -m 'Add amazing feature'`)
4. Alefaso any amin'ny tahiry ilay sampana (`git push origin feature/amazing-feature`)
5. Sokafy ny Pull Request

## Lisansa

Ity tetikasa ity dia fehezin'ny Lisansa MIT — jereo ny rakitra [LICENSE](../../../LICENSE) raha mila fanazavana fanampiny.

## Fisaorana

- Naka aingam-panahy avy amin'ny [ticker](https://github.com/achannarasappa/ticker)
- Naorina tamin'ny [ratatui](https://github.com/ratatui-org/ratatui)
- Angona avy amin'ny [Yahoo Finance](https://finance.yahoo.com/)
