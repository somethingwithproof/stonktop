# Stonktop

> [English](../../../README.md)

Kiolesura cha terminal kinachofanana na `top` kwa ajili ya kufuatilia bei za hisa na sarafu za kidijitali (cryptocurrency) kwa wakati halisi.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Vipengele

- **Ufuatiliaji wa Bei kwa Wakati Halisi**: Bei za moja kwa moja za hisa na sarafu za kidijitali
- **Kiolesura Kinachofanana na Top**: Vidhibiti vya kibodi vinavyojulikana vinavyofanana na `top`/`htop`
- **Chati za Sparkline**: Mwonekano wa mwenendo wa bei wa mstari mmoja kwenye upau wa pembeni
- **Ufuatiliaji wa Mfuko wa Uwekezaji (Portfolio)**: Fuatilia hisa na mali zako pamoja na hesabu ya faida na hasara (P/L)
- **Utafutaji na Uchujaji**: Utafutaji usiozingatia herufi kubwa au ndogo katika alama na majina
- **Arifa za Bei**: Arifa za kompyuta pindi bei inapovuka viwango vilivyowekwa
- **Hamisha/Leta (Export/Import)**: Hifadhi na pakia orodha za ufuatiliaji kama JSON au CSV
- **Sarafu Nyingi**: Onyesha bei katika sarafu yoyote kwa kutumia viwango vya ubadilishaji vya moja kwa moja
- **Chaguo Nyingi za Kupanga**: Panga kwa alama, bei, mabadiliko, kiasi cha biashara (volume), na thamani ya soko (market cap)
- **Hali ya Kundi (Batch Mode)**: Matokeo yasiyoingiliana kwa ajili ya uandishi wa hati otomatiki (kama `top -b`)
- **Usanidi wa TOML**: Faili za usanidi zilizo rahisi kuhariri
- **Uwekaji wa Rangi**: Kijani kwa faida, nyekundu kwa hasara
- **Njia za Mkato za Sarafu za Kidijitali**: Tumia `BTC.X` au `BTC` pekee badala ya `BTC-USD`

## Picha za Skrini

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

## Mahitaji ya Awali

- **Rust**: Toleo la 1.88 au la juu zaidi (MSRV)
- **Cargo**: Inajumuishwa katika usakinishaji wa Rust

## Usakinishaji

### Kutoka crates.io (Inapendekezwa)

```bash
cargo install stonktop
```

### Kwa Kutumia cargo-binstall

Ikiwa tayari umesakinisha [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), unaweza kusakinisha faili za binary zilizojengwa awali:

```bash
cargo binstall stonktop
```

### Faili za Binary Zilizojengwa Awali

Pakua faili za binary zilizojengwa awali kutoka ukurasa wa [Releases](https://github.com/somethingwithproof/stonktop/releases).

Mifumo inayopatikana:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Kutoka Msimbo Chanzo (From Source)

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Matumizi

### Matumizi ya Msingi

```bash
# Fuatilia alama mahususi
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Kwa kutumia njia za mkato za sarafu za kidijitali
stonktop -s AAPL,BTC,ETH,SOL
```

### Chaguo Zinazofanana na Top

```bash
# Weka muda wa kusubiri kabla ya kusasisha (kama top -d)
stonktop -s AAPL,GOOGL -d 10

# Endesha mizunguko N kisha ondoka (kama top -n)
stonktop -s AAPL -n 5

# Hali ya kundi kwa uandishi wa hati (kama top -b)
stonktop -s AAPL,GOOGL -b

# Hali salama - zima amri za mwingiliano
stonktop -s AAPL -S
```

### Chaguo za Kupanga

```bash
# Panga kwa bei (kushuka)
stonktop -s AAPL,GOOGL -o price

# Panga kwa alama (kupanda)
stonktop -s AAPL,GOOGL -o symbol -r

# Sehemu za kupanga zinazopatikana: symbol, name, price, change, change-percent, volume, market-cap
```

### Faili ya Usanidi

```bash
# Tumia faili mahususi ya usanidi
stonktop -c ~/.config/stonktop/custom.toml

# Au weka faili ya usanidi katika eneo la chaguo-msingi:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Chaguo za Mstari wa Amri (CLI)

| Chaguo | Kifupi | Maelezo |
|--------|--------|---------|
| `--symbols` | `-s` | Orodha ya alama za kufuatilia zilizotenganishwa kwa koma |
| `--delay` | `-d` | Muda wa kusasisha kwa sekunde (chaguo-msingi: 5) |
| `--iterations` | `-n` | Idadi ya mizunguko (0 = bila kikomo) |
| `--batch` | `-b` | Hali ya kundi - matokeo yasiyoingiliana |
| `--secure` | `-S` | Hali salama - zima amri za mwingiliano |
| `--config` | `-c` | Njia ya faili ya usanidi |
| `--sort` | `-o` | Sehemu ya awali ya kupanga |
| `--reverse` | `-r` | Geuza mpangilio wa kupanga |
| `--holdings` | `-H` | Onyesha mwonekano wa mali/mfuko wa uwekezaji |
| `--currency` | | Sarafu ya kuonyesha (chaguo-msingi: USD) |
| `--timeout` | | Muda wa mwisho wa API kwa sekunde (chaguo-msingi: 10) |
| `--format` | | Umbizo la matokeo kwa hali ya kundi: table, json, csv |
| `--color` | | Matokeo ya rangi: auto, always, never |
| `--verbose` | `-v` | Matokeo ya kina |
| `--init` | | Tengeneza faili ya mfano ya usanidi |
| `--force` | | Lazimisha kuandika upya usanidi uliopo (pamoja na `--init`) |
| `--lang` | | Lugha ya kuonyesha (`STONKTOP_LANG`, chaguo-msingi: otomatiki kutoka `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Onyesha ujumbe wa msaada |
| `--version` | `-V` | Onyesha toleo |

## Lugha Zinazotumika

Stonktop hutambua lugha ya kuonyesha kutoka `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, na `LANG`, au hukubali `--lang <LANG>`. Maeneo yote 45 ya lugha yanayotumika yanajumuisha tafsiri kamili za CLI na TUI (ikiwemo mpangilio wa kulia-hadi-kushoto kwa Kiarabu, Kiajemi, Kiebrania, na Kiurdu) pamoja na nyaraka zilizotafsiriwa:

| Msimbo | Lugha | README | Msimbo | Lugha | README | Msimbo | Lugha | README |
|--------|-------|--------|--------|-------|--------|--------|-------|--------|
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

## Njia za Mkato za Kibodi

| Kitufe | Kitendo |
|--------|---------|
| `q`, `Esc` | Ondoka |
| `h`, `?` | Washa/zima msaada |
| `↑`, `k` | Sogeza juu |
| `↓`, `j` | Sogeza chini |
| `g`, `Home` | Nenda juu kabisa |
| `G`, `End` | Nenda chini kabisa |
| `PgUp` | Ukurasa juu |
| `PgDn` | Ukurasa chini |
| `s` | Badilisha sehemu ya kupanga |
| `r` | Geuza mpangilio wa kupanga |
| `1-7` | Panga kwa safuwima |
| `Enter`, `d` | Onyesha dirisha la maelezo ya alama iliyochaguliwa |
| `/` | Tafuta/chuja alama |
| `a` | Ongeza alama kwenye orodha ya ufuatiliaji |
| `x` | Ondoa alama iliyochaguliwa |
| `S` | Washa/zima upau wa pembeni wa chati za sparkline |
| `e` | Hamisha orodha ya ufuatiliaji kwenda JSON |
| `H` | Washa/zima mwonekano wa mali |
| `f` | Washa/zima taarifa za msingi za kifedha (fundamentals) |
| `Space`, `R` | Lazimisha kusasisha |
| `Tab` | Badilisha kati ya makundi ya alama |

## Usanidi

Unda faili ya usanidi ya TOML:

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

# Mali za mfuko wa uwekezaji (si lazima)
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

# Makundi ya alama
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Onyesho la sarafu nyingi (si lazima)
[currency]
display = "EUR"
convert = true

# Arifa za bei (si lazima)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Njia za mkato maalum za sarafu za kidijitali (si lazima)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Miundo ya Alama

| Muundo | Maelezo | Mfano |
|--------|---------|-------|
| Hisa | Alama ya kawaida ya soko | `AAPL`, `GOOGL` |
| Sarafu ya kidijitali | Yenye kiambishi tamati cha USD | `BTC-USD`, `ETH-USD` |
| Kifupi cha sarafu ya kidijitali | Hupanuka kuwa -USD | `BTC.X` -> `BTC-USD` |
| Sarafu ya kidijitali otomatiki | Sarafu za kidijitali zinazojulikana | `BTC` -> `BTC-USD` |

## Chanzo cha Data

Stonktop hutumia Yahoo Finance API kupata bei za wakati halisi. Hakuna ufunguo wa API unaohitajika.

## Uendelezaji

### Ujenzi (Build)

```bash
# Ujenzi wa majaribio (Debug)
cargo build

# Ujenzi wa toleo kamili (Release)
cargo build --release
```

### Majaribio (Test)

```bash
# Endesha majaribio yote
cargo test

# Endesha majaribio ukionyesha matokeo
cargo test -- --nocapture
```

### Ukaguzi wa Msimbo (Lint)

```bash
# Kagua matatizo ya msimbo
cargo clippy --all-targets --all-features

# Tekeleza marekebisho ya otomatiki
cargo clippy --fix --allow-dirty
```

### Umbizo la Msimbo (Format)

```bash
# Kagua umbizo
cargo fmt -- --check

# Tekeleza umbizo
cargo fmt
```

## Kuchangia

Michango inakaribishwa! Jisikie huru kuwasilisha Pull Request.

1. Tengeneza nakala (Fork) ya hazina hii
2. Unda tawi lako la kipengele (`git checkout -b feature/amazing-feature`)
3. Hifadhi mabadiliko yako (`git commit -m 'Add amazing feature'`)
4. Tuma tawi lako (`git push origin feature/amazing-feature`)
5. Fungua Pull Request

## Leseni

Mradi huu umepewa leseni chini ya MIT License - angalia faili ya [LICENSE](../../../LICENSE) kwa maelezo zaidi.

## Shukrani

- Imehamasishwa na [ticker](https://github.com/achannarasappa/ticker)
- Imejengwa kwa [ratatui](https://github.com/ratatui-org/ratatui)
- Data kutoka [Yahoo Finance](https://finance.yahoo.com/)
