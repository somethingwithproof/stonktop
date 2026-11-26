# Stonktop

> [English](../../../README.md)

స్టాక్ మరియు క్రిప్టోకరెన్సీ ధరలను రియల్-టైమ్‌లో పర్యవేక్షించడానికి `top` వంటి టెర్మినల్ వినియోగదారు ఇంటర్‌ఫేస్ (TUI).

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## లక్షణాలు

- **రియల్-టైమ్ ధర పర్యవేక్షణ**: స్టాక్‌లు మరియు క్రిప్టోకరెన్సీల ప్రత్యక్ష ధరలు
- **`top` వంటి ఇంటర్‌ఫేస్**: `top`/`htop` తరహాలో సుపరిచితమైన కీబోర్డ్ నియంత్రణలు
- **స్పార్క్‌లైన్ చార్ట్‌లు**: సైడ్‌బార్‌లో తక్షణ ధరల ధోరణి దృశ్యీకరణ
- **పోర్ట్‌ఫోలియో ట్రాకింగ్**: లాభ/నష్టాల (P/L) గణనలతో మీ హోల్డింగ్‌లను పర్యవేక్షించండి
- **శోధన మరియు వడపోత**: సింబల్‌లు మరియు పేర్లలో అక్షరాల పరిమాణంతో నిమిత్తం లేని శోధన
- **ధర హెచ్చరికలు**: ధరలు నిర్ణయించిన పరిమితులను దాటినప్పుడు డెస్క్‌టాప్ నోటిఫికేషన్‌లు
- **ఎగుమతి/దిగుమతి**: వాచ్‌లిస్ట్‌లను JSON లేదా CSV ఫార్మాట్‌లో సేవ్ చేయండి మరియు లోడ్ చేయండి
- **బహుళ కరెన్సీ మద్దతు**: ప్రత్యక్ష మారకపు రేట్లతో ఏ కరెన్సీలోనైనా ధరలను ప్రదర్శించండి
- **బహుళ క్రమబద్ధీకరణ ఎంపికలు**: సింబల్, ధర, మార్పు, వాల్యూమ్, మార్కెట్ క్యాప్ ఆధారంగా క్రమబద్ధీకరించండి
- **బ్యాచ్ మోడ్**: స్క్రిప్టింగ్ కోసం ఇంటరాక్టివ్ కాని అవుట్‌పుట్ (`top -b` వలె)
- **TOML కాన్ఫిగరేషన్**: సులభంగా సవరించదగిన కాన్ఫిగరేషన్ ఫైల్‌లు
- **రంగు సంకేతాలు**: లాభాలకు ఆకుపచ్చ, నష్టాలకు ఎరుపు
- **క్రిప్టో షార్ట్‌కట్‌లు**: `BTC-USD` కోసం `BTC.X` లేదా కేవలం `BTC` ఉపయోగించండి

## స్క్రీన్‌షాట్లు

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

## ముందస్తు అవసరాలు

- **Rust**: వెర్షన్ 1.89 లేదా అంతకంటే ఎక్కువ (కనీస మద్దతు గల వెర్షన్ — MSRV)
- **Cargo**: Rust ఇన్‌స్టాలేషన్‌తో కలిపి వస్తుంది

## ఇన్‌స్టాలేషన్

### crates.io నుండి (సిఫార్సు చేయబడినది)

```bash
cargo install stonktop
```

### cargo-binstall ఉపయోగించి

మీ వద్ద [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) ఇన్‌స్టాల్ అయి ఉంటే, ముందుగా రూపొందించిన బైనరీలను ఇన్‌స్టాల్ చేయవచ్చు:

```bash
cargo binstall stonktop
```

### ముందుగా రూపొందించిన బైనరీలు

[Releases](https://github.com/somethingwithproof/stonktop/releases) పేజీ నుండి ముందుగా రూపొందించిన బైనరీలను డౌన్‌లోడ్ చేయండి.

అందుబాటులో ఉన్న ప్లాట్‌ఫారమ్‌లు:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### సోర్స్ కోడ్ నుండి

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## వినియోగం

### ప్రాథమిక వినియోగం

```bash
# నిర్దిష్ట సింబల్‌లను పర్యవేక్షించండి
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# క్రిప్టో షార్ట్‌కట్‌లతో
stonktop -s AAPL,BTC,ETH,SOL
```

### top వంటి ఎంపికలు

```bash
# రిఫ్రెష్ విరామాన్ని సెట్ చేయండి (top -d వలె)
stonktop -s AAPL,GOOGL -d 10

# N పునరావృతాలు అమలు చేసి నిష్క్రమించండి (top -n వలె)
stonktop -s AAPL -n 5

# స్క్రిప్టింగ్ కోసం బ్యాచ్ మోడ్ (top -b వలె)
stonktop -s AAPL,GOOGL -b

# సురక్షిత మోడ్ - ఇంటరాక్టివ్ కమాండ్‌లను నిలిపివేయండి
stonktop -s AAPL -S
```

### క్రమబద్ధీకరణ ఎంపికలు

```bash
# ధర ఆధారంగా క్రమబద్ధీకరించండి (అవరోహణ క్రమం)
stonktop -s AAPL,GOOGL -o price

# సింబల్ ఆధారంగా క్రమబద్ధీకరించండి (ఆరోహణ క్రమం)
stonktop -s AAPL,GOOGL -o symbol -r

# అందుబాటులో ఉన్న క్రమబద్ధీకరణ ఫీల్డ్‌లు: symbol, name, price, change, change-percent, volume, market-cap
```

### కాన్ఫిగరేషన్ ఫైల్

```bash
# నిర్దిష్ట కాన్ఫిగరేషన్ ఫైల్‌ను ఉపయోగించండి
stonktop -c ~/.config/stonktop/custom.toml

# లేదా డిఫాల్ట్ స్థానంలో కాన్ఫిగరేషన్ ఫైల్‌ను ఉంచండి:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## కమాండ్-లైన్ ఎంపికలు

| ఎంపిక | సంక్షిప్త రూపం | వివరణ |
|-------|---------------|-------|
| `--symbols` | `-s` | పర్యవేక్షించాల్సిన కామాతో వేరుచేసిన సింబల్‌ల జాబితా |
| `--delay` | `-d` | సెకన్లలో రిఫ్రెష్ విరామం (డిఫాల్ట్: 5) |
| `--iterations` | `-n` | పునరావృతాల సంఖ్య (0 = అనంతం) |
| `--batch` | `-b` | బ్యాచ్ మోడ్ — ఇంటరాక్టివ్ కాని అవుట్‌పుట్ |
| `--secure` | `-S` | సురక్షిత మోడ్ — ఇంటరాక్టివ్ కమాండ్‌లను నిలిపివేయండి |
| `--config` | `-c` | కాన్ఫిగరేషన్ ఫైల్ మార్గం |
| `--sort` | `-o` | ప్రారంభ క్రమబద్ధీకరణ ఫీల్డ్ |
| `--reverse` | `-r` | క్రమబద్ధీకరణ క్రమాన్ని తిప్పికొట్టండి |
| `--holdings` | `-H` | హోల్డింగ్‌లు/పోర్ట్‌ఫోలియో వీక్షణను చూపించు |
| `--currency` | | ప్రదర్శన కరెన్సీ (డిఫాల్ట్: USD) |
| `--timeout` | | సెకన్లలో API టైమ్‌అవుట్ (డిఫాల్ట్: 10) |
| `--format` | | బ్యాచ్ మోడ్ కోసం అవుట్‌పుట్ ఫార్మాట్: `table`, `json`, `csv` |
| `--color` | | రంగు అవుట్‌పుట్: `auto`, `always`, `never` |
| `--verbose` | `-v` | వివరమైన అవుట్‌పుట్ |
| `--init` | | నమూనా కాన్ఫిగరేషన్ ఫైల్‌ను రూపొందించండి |
| `--force` | | ఇప్పటికే ఉన్న కాన్ఫిగరేషన్‌ను బలవంతంగా ఓవర్‌రైట్ చేయండి (`--init` తో) |
| `--lang` | | ప్రదర్శన భాష (`STONKTOP_LANG`, డిఫాల్ట్: `LC_ALL`/`LC_MESSAGES`/`LANG` నుండి స్వయంచాలకం) |
| `--help` | | సహాయ సందేశాన్ని చూపించు |
| `--version` | `-V` | వెర్షన్‌ను చూపించు |

## మద్దతు ఉన్న భాషలు

Stonktop ప్రదర్శన భాషను `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, మరియు `LANG` నుండి గుర్తిస్తుంది, లేదా `--lang <LANG>` ఎంపికను స్వీకరిస్తుంది. మద్దతు ఉన్న మొత్తం 48 లోకేల్స్‌లో పూర్తి CLI మరియు TUI అనువాదాలు (అరబిక్, పర్షియన్, హిబ్రూ మరియు ఉర్దూ భాషలకు కుడి నుండి ఎడమకు అమరికతో సహా) మరియు స్థానికీకరించిన డాక్యుమెంటేషన్ ఉన్నాయి:

| కోడ్ | భాష | README | కోడ్ | భాష | README | కోడ్ | భాష | README |
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
| `ko` | 한국어 | [한국어](../ko/README.md) | `so-SO` | Soomaali | [Soomaali](../so-SO/README.md) | `bs` | Bosanski | [Bosanski](../bs/README.md) |
| `zh-CN` | 简体中文 | [简体中文](../zh-CN/README.md) | `sw-KE` | Kiswahili | [Kiswahili](../sw-KE/README.md) | `hr` | Hrvatski | [Hrvatski](../hr/README.md) |
| `zh-TW` | 繁體中文 | [繁體中文](../zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](../xh-ZA/README.md) | `sr` | Српски | [Српски](../sr/README.md) |
| `ar` | العربية | [العربية](../ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](../yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](../hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](../zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](../pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](../tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](../sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](../vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](../id/README.md) | | | | | | |

## కీబోర్డ్ షార్ట్‌కట్లు

| కీ | చర్య |
|----|------|
| `q`, `Esc` | నిష్క్రమించు |
| `h`, `?` | సహాయాన్ని టోగుల్ చేయి |
| `↑`, `k` | పైకి వెళ్ళు |
| `↓`, `j` | కిందికి వెళ్ళు |
| `g`, `Home` | పైభాగానికి వెళ్ళు |
| `G`, `End` | దిగువకు వెళ్ళు |
| `PgUp` | ఒక పేజీ పైకి |
| `PgDn` | ఒక పేజీ కిందికి |
| `s` | క్రమబద్ధీకరణ ఫీల్డ్‌ను మార్చు |
| `r` | క్రమబద్ధీకరణ క్రమాన్ని తిప్పికొట్టు |
| `1-7` | కాలమ్ ఆధారంగా క్రమబద్ధీకరించు |
| `Enter` | ఎంచుకున్న సింబల్ వివరాల పాప్‌అప్‌ను చూపించు |
| `/` | సింబల్‌లను శోధించు/వడపోయి |
| `a` | వాచ్‌లిస్ట్‌కు సింబల్‌ను జోడించు |
| `d` | ఎంచుకున్న సింబల్‌ను తొలగించు |
| `S` | స్పార్క్‌లైన్ చార్ట్‌ల సైడ్‌బార్‌ను టోగుల్ చేయి |
| `e` | వాచ్‌లిస్ట్‌ను JSON కు ఎగుమతి చేయి |
| `H` | హోల్డింగ్‌ల వీక్షణను టోగుల్ చేయి |
| `f` | ప్రాథమిక ఆర్థిక వివరాలను (ఫండమెంటల్స్) టోగుల్ చేయి |
| `Space`, `R` | బలవంతంగా రిఫ్రెష్ చేయి |
| `Tab` | సింబల్ సమూహాల మధ్య మారు |

## కాన్ఫిగరేషన్

TOML కాన్ఫిగరేషన్ ఫైల్‌ను రూపొందించండి:

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

# పోర్ట్‌ఫోలియో హోల్డింగ్‌లు (ఐచ్ఛికం)
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

# సింబల్ సమూహాలు
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# బహుళ కరెన్సీ ప్రదర్శన (ఐచ్ఛికం)
[currency]
display = "EUR"
convert = true

# ధర హెచ్చరికలు (ఐచ్ఛికం)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# కస్టమ్ క్రిప్టో షార్ట్‌కట్‌లు (ఐచ్ఛికం)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## సింబల్ ఫార్మాట్లు

| ఫార్మాట్ | వివరణ | ఉదాహరణ |
|----------|-------|--------|
| స్టాక్ | ప్రామాణిక టిక్కర్ | `AAPL`, `GOOGL` |
| క్రిప్టో | USD ప్రత్యయంతో | `BTC-USD`, `ETH-USD` |
| క్రిప్టో సంక్షిప్తం | `-USD` కు విస్తరిస్తుంది | `BTC.X` -> `BTC-USD` |
| క్రిప్టో ఆటో | సాధారణ క్రిప్టోకరెన్సీలు | `BTC` -> `BTC-USD` |

## డేటా మూలం

రియల్-టైమ్ ధరలను పొందడానికి Stonktop Yahoo Finance API ని ఉపయోగిస్తుంది. ఎటువంటి API కీ అవసరం లేదు.

## అభివృద్ధి

### బిల్డ్

```bash
# డీబగ్ బిల్డ్
cargo build

# రిలీజ్ బిల్డ్
cargo build --release
```

### పరీక్ష

```bash
# అన్ని పరీక్షలను అమలు చేయండి
cargo test

# అవుట్‌పుట్‌తో పరీక్షలను అమలు చేయండి
cargo test -- --nocapture
```

### లింట్

```bash
# లింటింగ్ సమస్యలను తనిఖీ చేయండి
cargo clippy --all-targets --all-features

# స్వయంచాలక పరిష్కారాలను వర్తింపజేయండి
cargo clippy --fix --allow-dirty
```

### ఫార్మాట్

```bash
# ఫార్మాటింగ్‌ను తనిఖీ చేయండి
cargo fmt -- --check

# ఫార్మాటింగ్‌ను వర్తింపజేయండి
cargo fmt
```

## సహకారం

సహకారాలు ఎల్లప్పుడూ స్వాగతించబడతాయి! దయచేసి సంకోచించకుండా Pull Request సమర్పించండి.

1. రిపోజిటరీని ఫోర్క్ చేయండి
2. మీ ఫీచర్ బ్రాంచ్‌ను సృష్టించండి (`git checkout -b feature/amazing-feature`)
3. మీ మార్పులను కమిట్ చేయండి (`git commit -m 'Add amazing feature'`)
4. బ్రాంచ్‌కు పుష్ చేయండి (`git push origin feature/amazing-feature`)
5. Pull Request ను తెరవండి

## లైసెన్స్

ఈ ప్రాజెక్ట్ MIT లైసెన్స్ కింద లైసెన్స్ పొందింది — వివరాల కోసం [LICENSE](../../../LICENSE) ఫైల్‌ను చూడండి.

## కృతజ్ఞతలు

- [ticker](https://github.com/achannarasappa/ticker) నుండి ప్రేరణ పొందింది
- [ratatui](https://github.com/ratatui-org/ratatui) తో నిర్మించబడింది
- డేటా మూలం: [Yahoo Finance](https://finance.yahoo.com/)
