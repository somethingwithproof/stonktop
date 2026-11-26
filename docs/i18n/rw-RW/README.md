# Stonktop

> [English](../../../README.md)

Porogaramu yo muri terminal imeze nka `top` yo gukurikirana ibiciro by'imigabane n'amafaranga y'ikoranabuhanga (cryptocurrency) ako kanya.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Ibiranga Porogaramu

- **Gukurikirana Ibiciro Ako Kanya**: Ibiciro by'ako kanya by'imigabane n'amafaranga y'ikoranabuhanga
- **Imigaragarire Imeze nka `top`**: Uburyo bumenyerewe bwo kuyoboresha klaviye nka `top`/`htop`
- **Imbonerahamwe Ntoya za Sparkline**: Kugaragaza icyerekezo cy'ibiciro mu gice cyo ku ruhande
- **Gukurikirana Umutungo (Portfolio)**: Kurikirana imigabane yawe hamwe n'imibare y'inyungu n'igihombo (P/L)
- **Gushakisha no Kuyungurura**: Gushakisha bititaye ku nyuguti nkuru cyangwa nto mu bimenyetso n'amazina
- **Imenyesha ry'Ibiciro**: Ubutumwa bumenyesha kuri mudasobwa igihe ibiciro birenze igipimo wateganyije
- **Kohereza no Kwinjiza**: Bika kandi ufungure urutonde rw'ibyo ukurikirana mu buryo bwa JSON cyangwa CSV
- **Amafaranga Atandukanye**: Garagaza ibiciro mu ifaranga iryo ari ryo ryose hamwe n'igipimo cy'ivunjisha ako kanya
- **Uburyo Bwinshi bwo Gutondeka**: Tondeka ukurikije ikimenyetso, igiciro, impinduka, ingano y'ubucuruzi, cyangwa agaciro ku isoko
- **Uburyo bwa Batch**: Ibisohoka bitari iby'imikoranire bigenewe porogaramu zikora zonyine (nka `top -b`)
- **Igenamiterere rya TOML**: Dosiye z'igenamiterere zoroshye guhindura
- **Ibimenyetso by'Amabara**: Icyatsi kibisi ku nyungu, umutuku ku gihombo
- **Inzira Ngufi za Crypto**: Koresha `BTC.X` cyangwa `BTC` gusa mu mwanya wa `BTC-USD`

## Amafoto y'Imikorere

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

## Ibisabwa Mbere

- **Rust**: Verisiyo 1.89 cyangwa irenzeho (MSRV)
- **Cargo**: Izanana n'ishyirwaho rya Rust

## Ishyirwaho (Installation)

### Bivuye kuri crates.io (Ibyifuzwa)

```bash
cargo install stonktop
```

### Ukoresheje cargo-binstall

Niba ufite [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) yashyizweho, ushobora gushyiraho porogaramu zakozwe mbere:

```bash
cargo binstall stonktop
```

### Porogaramu Zakozwe Mbere

Kurura porogaramu zakozwe mbere ku rupapuro rwa [Releases](https://github.com/somethingwithproof/stonktop/releases).

Sisitemu ziboneka:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Bivuye ku Nkunga ya Kode (Source)

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Imikoreshereze

### Imikoreshereze y'Ibanze

```bash
# Kurikirana ibimenyetso byihariye
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Ukoresheje inzira ngufi z'amafaranga y'ikoranabuhanga
stonktop -s AAPL,BTC,ETH,SOL
```

### Amahitamo Ameze nka `top`

```bash
# Gena igihe cyo kuvugurura (nka top -d)
stonktop -s AAPL,GOOGL -d 10

# Kora inshuro N hanyuma usohoke (nka top -n)
stonktop -s AAPL -n 5

# Uburyo bwa batch bwa za script (nka top -b)
stonktop -s AAPL,GOOGL -b

# Uburyo bw'umutekano - guhagarika amabwiriza y'imikoranire
stonktop -s AAPL -S
```

### Amahitamo yo Gutondeka

```bash
# Tondeka ukurikije igiciro (uva hejuru ujya hasi)
stonktop -s AAPL,GOOGL -o price

# Tondeka ukurikije ikimenyetso (uva hasi ujya hejuru)
stonktop -s AAPL,GOOGL -o symbol -r

# Ibyiciro byo gutondeka biboneka: symbol, name, price, change, change-percent, volume, market-cap
```

### Dosiye y'Igenamiterere

```bash
# Koresha dosiye y'igenamiterere yihariye
stonktop -c ~/.config/stonktop/custom.toml

# Cyangwa shyira dosiye aho isanzwe iba:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Amahitamo y'Umurongo w'Amabwiriza

| Ihitamo | Impine | Ibisobanuro |
|---------|--------|-------------|
| `--symbols` | `-s` | Urutonde rw'ibimenyetso byo gukurikirana bitandukanyijwe n'ikitso |
| `--delay` | `-d` | Igihe cyo kuvugurura mu masegonda (gisanzwe: 5) |
| `--iterations` | `-n` | Umubare w'inshuro zo kuvugurura (0 = zidashira) |
| `--batch` | `-b` | Uburyo bwa batch - ibisohoka bitari iby'imikoranire |
| `--secure` | `-S` | Uburyo bw'umutekano - guhagarika amabwiriza y'imikoranire |
| `--config` | `-c` | Inzira ya dosiye y'igenamiterere |
| `--sort` | `-o` | Icyiciro cy'ibanze cyo gutondeka |
| `--reverse` | `-r` | Hindura icyerekezo cyo gutondeka |
| `--holdings` | `-H` | Garagaza imigaragarire y'umutungo/imigabane |
| `--currency` | | Ifaranga ryo kugaragaza ibiciro (risanzwe: USD) |
| `--timeout` | | Igihe ntarengwa cyo gutegereza API mu masegonda (gisanzwe: 10) |
| `--format` | | Imiterere y'ibisohoka mu buryo bwa batch: table, json, csv |
| `--color` | | Amabara y'ibisohoka: auto, always, never |
| `--verbose` | `-v` | Ibisohoka birambuye |
| `--init` | | Kora dosiye y'urugero rw'igenamiterere |
| `--force` | | Hatira gusimbura dosiye y'igenamiterere ihari (hamwe na `--init`) |
| `--lang` | | Ururimi rugaragara (`STONKTOP_LANG`, gisanzwe: rwitahura ruturutse kuri `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Garagaza ubutumwa bw'ubufasha |
| `--version` | `-V` | Garagaza verisiyo |

## Indimi Zishyigikiwe

Stonktop itahura ururimi rugaragara ikuye kuri `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, na `LANG`, cyangwa ikemera `--lang <LANG>`. Indimi zose 48 zishyigikiwe zirimo ubusemuzi bwuzuye bwa CLI na TUI (harimo n'imyandikire iva iburyo ijya ibumoso ku Cyarabu, Igiperesi, Igiheburayo, n'Iki-Urdu) hamwe n'inyandiko zisemuye:

| Kode | Ururimi | README | Kode | Ururimi | README | Kode | Ururimi | README |
|------|---------|--------|------|---------|--------|------|---------|--------|
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

## Inzira Ngufi za Klaviye

| Urufunguzo | Igikorwa |
|------------|----------|
| `q`, `Esc` | Sohoka |
| `h`, `?` | Garagaza cyangwa hisha ubufasha |
| `↑`, `k` | Zamuka hejuru |
| `↓`, `j` | Manuka hasi |
| `g`, `Home` | Jya ku ntangiriro |
| `G`, `End` | Jya ku mpera |
| `PgUp` | Urupapuro rubanza |
| `PgDn` | Urupapuro rukurikira |
| `s` | Hinduranya icyiciro cyo gutondeka |
| `r` | Hindura icyerekezo cyo gutondeka |
| `1-7` | Tondeka ukurikije inkingi |
| `Enter` | Garagaza idirishya ry'amakuru arambuye y'ikimenyetso cyahiswemo |
| `/` | Shakisha cyangwa yungurura ibimenyetso |
| `a` | Ongera ikimenyetso ku rutonde rw'ibikurikiranwa |
| `d` | Kuraho ikimenyetso cyahiswemo |
| `S` | Garagaza cyangwa hisha agace k'imbonerahamwe ntoya za sparkline |
| `e` | Ohereza urutonde rw'ibikurikiranwa muri JSON |
| `H` | Hindura imigaragarire y'umutungo |
| `f` | Garagaza cyangwa hisha amakuru y'ibanze (fundamentals) |
| `Space`, `R` | Hatira ivugurura |
| `Tab` | Hinduranya amatsinda y'ibimenyetso |

## Igenamiterere

Kora dosiye y'igenamiterere ya TOML:

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

# Imigabane ifitwe mu mutungo (ntibisabwa)
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

# Amatsinda y'ibimenyetso
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Kugaragaza mu mafaranga atandukanye (ntibisabwa)
[currency]
display = "EUR"
convert = true

# Imenyesha ry'ibiciro (ntibisabwa)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Inzira ngufi za crypto zihariye (ntibisabwa)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Imiterere y'Ibimenyetso

| Imiterere | Ibisobanuro | Urugero |
|-----------|-------------|---------|
| Imigabane | Ikimenyetso gisanzwe cy'isoko | `AAPL`, `GOOGL` |
| Amafaranga y'ikoranabuhanga | Hamwe n'umusozo wa USD | `BTC-USD`, `ETH-USD` |
| Impine ya crypto | Ihinduka `-USD` | `BTC.X` -> `BTC-USD` |
| Crypto yihindura ubwayo | Amafaranga y'ikoranabuhanga akoreshwa cyane | `BTC` -> `BTC-USD` |

## Inkomoko y'Amakuru

Stonktop ikoresha Yahoo Finance API mu gukura ibiciro by'ako kanya. Nta rufunguzo rwa API (API key) rusabwa.

## Iterambere

### Kubaka (Build)

```bash
# Kubaka mu buryo bwa debug
cargo build

# Kubaka mu buryo bwa release
cargo build --release
```

### Isuzuma (Test)

```bash
# Koresha amasuzuma yose
cargo test

# Koresha amasuzuma ugaragaza ibisohoka
cargo test -- --nocapture
```

### Isuzuma ry'Imyandikire ya Kode (Lint)

```bash
# Genzura amakosa ya lint
cargo clippy --all-targets --all-features

# Shyiraho ibikosorwa byikora
cargo clippy --fix --allow-dirty
```

### Imitunganyirize ya Kode (Format)

```bash
# Genzura imitunganyirize ya kode
cargo fmt -- --check

# Shyiraho imitunganyirize ya kode
cargo fmt
```

## Gutanga Umusanzu

Imisanzu irahawe ikaze! Ushobora kohereza Pull Request igihe cyose.

1. Kora Fork ya repository
2. Kora ishami ry'igikorwa gishya (`git checkout -b feature/amazing-feature`)
3. Emeza impinduka zawe (`git commit -m 'Add amazing feature'`)
4. Ohereza ishami ryawe (`git push origin feature/amazing-feature`)
5. Fungura Pull Request

## Uruhushya

Uyu mushinga ufite uruhushya rwa MIT — reba dosiye ya [LICENSE](../../../LICENSE) ku bisobanuro birambuye.

## Ishimwe

- Byavuye ku gitekerezo cya [ticker](https://github.com/achannarasappa/ticker)
- Byubatswe hakoreshejwe [ratatui](https://github.com/ratatui-org/ratatui)
- Amakuru aturuka kuri [Yahoo Finance](https://finance.yahoo.com/)
