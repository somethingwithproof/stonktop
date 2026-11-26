# Stonktop

> [English](../../../README.md)

Ngwaọrụ ihuenyo temịnalụ yiri `top` maka ileba anya n'ọnụ ahịa òkè ahịa na ego dijitalụ (cryptocurrency) n'oge kwesịrị ekwesị.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Atụmatụ

- **Nleba Anya Ọnụ Ahịa n'Oge Ugbu a**: Ọnụ ahịa dị ndụ maka òkè ahịa na ego dijitalụ
- **Ihuenyo Yiri `top`**: Njikwa ahụigodo a maara nke ọma dị ka `top`/`htop`
- **Chaatị Sparkline**: Ngosipụta usoro mgbanwe ọnụ ahịa n'ogige akụkụ
- **Nleba Anya Pọtụfoliyo**: Soro ngwa ahịa gị yana mgbakọ uru na mfu (P/L)
- **Ọchụchọ na Nzacha**: Ọchụchọ na-adịghị elebara nnukwu ma ọ bụ obere mkpụrụedemede anya n'akara na aha
- **Ọkwa Ọnụ Ahịa**: Ozi ngosi na desktọpụ mgbe ọnụ ahịa gafere oke a haziri
- **Mbupụ/Mbubata**: Chekwaa ma bubata ndepụta nleba anya dị ka JSON ma ọ bụ CSV
- **Ọtụtụ Ụdị Ego**: Gosipụta ọnụ ahịa n'ụdị ego ọ bụla nwere ọnụego mgbanwe dị ndụ
- **Ọtụtụ Nhọrọ Nhazi Usoro**: Hazie site n'akara, ọnụ ahịa, mgbanwe, oke azụmahịa, ma ọ bụ isi ego ahịa
- **Ọnọdụ Otu Oge (Batch Mode)**: Mpụta na-enweghị mmekọrịta maka edemede akpaaka (dị ka `top -b`)
- **Nhazi TOML**: Faịlụ nhazi dị mfe idezi
- **Agba Ngosi**: Akwụkwọ ndụ maka uru, uhie maka mfu
- **Mkpirisi Ego Dijitalụ**: Jiri `BTC.X` ma ọ bụ naanị `BTC` maka `BTC-USD`

## Foto Ihuenyo

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

## Ihe Ndị A Chọrọ Mbụ

- **Rust**: Ụdị 1.89 ma ọ bụ karịa (MSRV)
- **Cargo**: Etinyere ya na nwụnye Rust

## Nwụnye

### Site na crates.io (A Kwadoro)

```bash
cargo install stonktop
```

### Site n'Iji cargo-binstall

Ọ bụrụ na ị wụnyerela [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), ị nwere ike ịwụnye faịlụ e wuru tupu oge eruo:

```bash
cargo binstall stonktop
```

### Faịlụ E Wuru Tupu Oge Eruo

Budata faịlụ e wuru tupu oge eruo site na ibe [Releases](https://github.com/somethingwithproof/stonktop/releases).

Usoro arụmọrụ dịnụ:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Site na Koodu Isi Mmalite

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Ojiji

### Ojiji Ntọala

```bash
# Lelee akara ụfọdụ kpọmkwem
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Jiri mkpirisi ego dijitalụ
stonktop -s AAPL,BTC,ETH,SOL
```

### Nhọrọ Yiri `top`

```bash
# Hazie oge mmegharị (dị ka top -d)
stonktop -s AAPL,GOOGL -d 10

# Gbaa okirikiri N wee pụọ (dị ka top -n)
stonktop -s AAPL -n 5

# Ọnọdụ batch maka edemede akpaaka (dị ka top -b)
stonktop -s AAPL,GOOGL -b

# Ọnọdụ nchekwa - gbanyụọ iwu mmekọrịta
stonktop -s AAPL -S
```

### Nhọrọ Nhazi Usoro

```bash
# Hazie site n'ọnụ ahịa (site n'elu gbadaa)
stonktop -s AAPL,GOOGL -o price

# Hazie site n'akara (site n'ala gbagoo)
stonktop -s AAPL,GOOGL -o symbol -r

# Mpaghara nhazi dịnụ: symbol, name, price, change, change-percent, volume, market-cap
```

### Faịlụ Nhazi

```bash
# Jiri faịlụ nhazi kpọmkwem
stonktop -c ~/.config/stonktop/custom.toml

# Ma ọ bụ tinye faịlụ nhazi n'ebe ndabara:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Nhọrọ Ahịrị Iwu

| Nhọrọ | Mkpirisi | Nkọwa |
|-------|----------|-------|
| `--symbols` | `-s` | Ndepụta akara a ga-eleba anya nke rikọm kewara |
| `--delay` | `-d` | Oge nkwụsịtụ mmegharị na sekọnd (ndabara: 5) |
| `--iterations` | `-n` | Ọnụọgụgụ okirikiri mmegharị (0 = enweghị njedebe) |
| `--batch` | `-b` | Ọnọdụ batch - mpụta na-enweghị mmekọrịta |
| `--secure` | `-S` | Ọnọdụ nchekwa - gbanyụọ iwu mmekọrịta |
| `--config` | `-c` | Ụzọ ebe faịlụ nhazi dị |
| `--sort` | `-o` | Mpaghara nhazi mbụ |
| `--reverse` | `-r` | Tụgharịa usoro nhazi |
| `--holdings` | `-H` | Gosipụta nlele ngwa ahịa/pọtụfoliyo |
| `--currency` | | Ụdị ego ngosi (ndabara: USD) |
| `--timeout` | | Oke oge nchere API na sekọnd (ndabara: 10) |
| `--format` | | Ụdị mpụta maka ọnọdụ batch: table, json, csv |
| `--color` | | Mpụta agba: auto, always, never |
| `--verbose` | `-v` | Mpụta zuru ezu |
| `--init` | | Mepụta faịlụ nhazi atụ |
| `--force` | | Manye idegharị nhazi dị adị (ya na `--init`) |
| `--lang` | | Asụsụ ngosi (`STONKTOP_LANG`, ndabara: nchọpụta akpaaka site na `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Gosipụta ozi enyemaka |
| `--version` | `-V` | Gosipụta ụdị mbipụta |

## Asụsụ A Kwadoro

Stonktop na-achọpụta asụsụ ngosi site na `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, na `LANG`, ma ọ bụ na-anabata `--lang <LANG>`. Asụsụ 45 niile a kwadoro gụnyere ntụgharị asụsụ CLI na TUI zuru oke (gụnyere nhazi aka nri gaa aka ekpe maka Arabic, Persian, Hebrew, na Urdu) yana akwụkwọ nkọwa n'asụsụ ndị ahụ:

| Koodu | Asụsụ | README | Koodu | Asụsụ | README | Koodu | Asụsụ | README |
|-------|-------|--------|-------|-------|--------|-------|-------|--------|
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

## Mkpirisi Ahụigodo

| Igodo | Omume |
|-------|-------|
| `q`, `Esc` | Pụọ |
| `h`, `?` | Gbanwee ngosi enyemaka |
| `↑`, `k` | Gaa elu |
| `↓`, `j` | Gaa ala |
| `g`, `Home` | Gaa n'isi elu |
| `G`, `End` | Gaa n'ala ala |
| `PgUp` | Ibe elu |
| `PgDn` | Ibe ala |
| `s` | Gbanwee mpaghara nhazi usoro |
| `r` | Tụgharịa usoro nhazi |
| `1-7` | Hazie site na kọlụm |
| `Enter` | Gosipụta windo nkọwa zuru ezu maka akara a họọrọ |
| `/` | Chọọ ma ọ bụ zacha akara |
| `a` | Tinye akara na ndepụta nleba anya |
| `d` | Wepụ akara a họọrọ |
| `S` | Gbanwee ngosi ogige akụkụ chaatị sparkline |
| `e` | Bupụ ndepụta nleba anya gaa na JSON |
| `H` | Gbanwee nlele ngwa ahịa |
| `f` | Gbanwee ngosi data ntọala (fundamentals) |
| `Space`, `R` | Manye mmegharị |
| `Tab` | Gbanwee n'etiti otu akara |

## Nhazi

Mepụta faịlụ nhazi TOML:

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

# Ngwa ahịa pọtụfoliyo (nhọrọ)
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

# Otu akara
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Ngosi ọtụtụ ụdị ego (nhọrọ)
[currency]
display = "EUR"
convert = true

# Ọkwa ọnụ ahịa (nhọrọ)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Mkpirisi ego dijitalụ ahaziri iche (nhọrọ)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Ụdị Akara

| Ụdị | Nkọwa | Ọmụmaatụ |
|-----|-------|----------|
| Òkè ahịa | Akara ahịa ọkọlọtọ | `AAPL`, `GOOGL` |
| Ego dijitalụ | Nwere nsonaazụ USD | `BTC-USD`, `ETH-USD` |
| Mkpirisi ego dijitalụ | Na-agbasa gaa na `-USD` | `BTC.X` -> `BTC-USD` |
| Ego dijitalụ akpaaka | Ego dijitalụ a na-ahụkarị | `BTC` -> `BTC-USD` |

## Isi Mmalite Data

Stonktop na-eji Yahoo Finance API enweta ọnụ ahịa n'oge ugbu a. A chọghị igodo API ọ bụla.

## Mmepe

### Iwulite

```bash
# Iwulite maka nchọpụta njehie (Debug)
cargo build

# Iwulite maka mbipụta (Release)
cargo build --release
```

### Nnwale

```bash
# Gbaa nnwale niile
cargo test

# Gbaa nnwale na ngosi mpụta
cargo test -- --nocapture
```

### Nnyocha Koodu (Lint)

```bash
# Lelee nsogbu nnyocha koodu
cargo clippy --all-targets --all-features

# Tinye ndozi akpaaka
cargo clippy --fix --allow-dirty
```

### Nhazi Ụdị Koodu

```bash
# Lelee nhazi ụdị koodu
cargo fmt -- --check

# Tinye nhazi ụdị koodu
cargo fmt
```

## Ntụnye

A na-anabata ntụnye n'ọṅụ! Biko nwee nnwere onwe izipu Pull Request.

1. Mee Fork nke ebe nchekwa koodu ahụ
2. Mepụta alaka atụmatụ gị (`git checkout -b feature/amazing-feature`)
3. Chekwaa mgbanwe gị (`git commit -m 'Add amazing feature'`)
4. Ziga alaka ahụ (`git push origin feature/amazing-feature`)
5. Mepee Pull Request

## Ikikere

A nyere ọrụ a ikikere n'okpuru Ikikere MIT — lee faịlụ [LICENSE](../../../LICENSE) maka nkọwa zuru ezu.

## Ekele

- Sitere n'ike mmụọ nsọ nke [ticker](https://github.com/achannarasappa/ticker)
- E wulitere ya site na [ratatui](https://github.com/ratatui-org/ratatui)
- Data sitere na [Yahoo Finance](https://finance.yahoo.com/)
