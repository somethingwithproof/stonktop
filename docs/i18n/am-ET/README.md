# Stonktop

> [English](../../../README.md)

የአክሲዮን እና የክሪፕቶ ምንዛሬ ዋጋዎችን በቅጽበት ለመከታተል የሚያገለግል እንደ `top` ያለ የተርሚናል የተጠቃሚ በይነገጽ (TUI)።

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## ዋና ዋና ባህሪያት

- **የቅጽበታዊ ዋጋ ክትትል**፦ ለአክሲዮኖች እና ለክሪፕቶ ምንዛሬዎች የቀጥታ የዋጋ ተመኖች
- **እንደ `top` ያለ በይነገጽ**፦ ከ `top`/`htop` ጋር ተመሳሳይ የሆነ የተለመደ የቁልፍ ሰሌዳ መቆጣጠሪያ
- **የስፓርክላይን ገበታዎች**፦ በጎን አሞሌ ውስጥ የዋጋ አዝማሚያን በምስል የሚያሳዩ መስመራዊ ገበታዎች
- **የፖርትፎሊዮ ክትትል**፦ የያዟቸውን ንብረቶች ከትርፍና ኪሳራ ስሌት ጋር ይከታተሉ
- **ፍለጋ እና ማጣሪያ**፦ በምልክቶች እና በስሞች ላይ የፊደል መጠን የማይለይ ፍለጋ
- **የዋጋ ማንቂያዎች**፦ ዋጋዎች የተቀመጡ ገደቦችን ሲያልፉ የዴስክቶፕ ማሳወቂያዎችን ያቀርባል
- **መላክ እና ማስገባት**፦ የክትትል ዝርዝሮችን በ JSON ወይም CSV ቅርጸት ያስቀምጡና ይጫኑ
- **ብዙ ምንዛሬዎች**፦ በቀጥታ የምንዛሬ ተመን ልወጣ ዋጋዎችን በማንኛውም ምንዛሬ ያሳያል
- **ብዙ የመደርደሪያ አማራጮች**፦ በምልክት፣ በዋጋ፣ በለውጥ፣ በግብይት መጠን እና በገበያ ዋጋ መደርደር
- **የባች ሁነታ**፦ ለስክሪፕት አጠቃቀም መስተጋብራዊ ያልሆነ ውጤት (እንደ `top -b`)
- **የ TOML ማዋቀሪያ**፦ ለማስተካከል ቀላል የሆኑ የማዋቀሪያ ፋይሎች
- **የቀለም ምልክት**፦ ለጭማሪ አረንጓዴ፣ ለቅናሽ ቀይ ቀለም
- **የክሪፕቶ አቋራጮች**፦ ለ `BTC-USD` በቀጥታ `BTC.X` ወይም `BTC` ብቻ ይጠቀሙ

## የማያ ገጽ ቅጽበታዊ ገጽ እይታዎች

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

## ቅድመ ሁኔታዎች

- **Rust**፦ ስሪት 1.89 ወይም ከዚያ በላይ (ዝቅተኛው የሚደገፍ የ Rust ስሪት)
- **Cargo**፦ ከ Rust ጭነት ጋር አብሮ ይመጣል

## መጫን

### ከ crates.io (የሚመከር)

```bash
cargo install stonktop
```

### cargo-binstall በመጠቀም

[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) በስርዓትዎ ላይ ከተጫነ፣ ቀድመው የተገነቡ ሁለትዮሽ ፋይሎችን በቀጥታ መጫን ይችላሉ፦

```bash
cargo binstall stonktop
```

### ቀድመው የተገነቡ ሁለትዮሽ ፋይሎች

ቀድመው የተገነቡ ሁለትዮሽ ፋይሎችን ከ[ልቀቶች](https://github.com/somethingwithproof/stonktop/releases) ገጽ ያውርዱ።

የሚገኙ መድረኮች፦
- Linux (x86_64፣ aarch64፣ musl)
- macOS (Intel፣ Apple Silicon)
- Windows (x86_64፣ aarch64)

### ከምንጭ ኮድ መገንባት

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## አጠቃቀም

### መሠረታዊ አጠቃቀም

```bash
# የተወሰኑ ምልክቶችን ይከታተሉ
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# የክሪፕቶ አቋራጮችን በመጠቀም
stonktop -s AAPL,BTC,ETH,SOL
```

### እንደ top ያሉ አማራጮች

```bash
# የማደሻ ጊዜ ገደብ ያቀናብሩ (እንደ top -d)
stonktop -s AAPL,GOOGL -d 10

# N ድግግሞሾችን ካከናወነ በኋላ ይውጣ (እንደ top -n)
stonktop -s AAPL -n 5

# ለስክሪፕት አጠቃቀም የባች ሁነታ (እንደ top -b)
stonktop -s AAPL,GOOGL -b

# ደህንነቱ የተጠበቀ ሁነታ - መስተጋብራዊ ትዕዛዞችን ያሰናክላል
stonktop -s AAPL -S
```

### የመደርደሪያ አማራጮች

```bash
# በዋጋ ደርድር (ከከፍተኛ ወደ ዝቅተኛ)
stonktop -s AAPL,GOOGL -o price

# በምልክት ደርድር (ከዝቅተኛ ወደ ከፍተኛ)
stonktop -s AAPL,GOOGL -o symbol -r

# የሚገኙ የመደርደሪያ መስኮች፦ symbol, name, price, change, change-percent, volume, market-cap
```

### የማዋቀሪያ ፋይል

```bash
# የተወሰነ የማዋቀሪያ ፋይል ይጠቀሙ
stonktop -c ~/.config/stonktop/custom.toml

# ወይም የማዋቀሪያ ፋይሉን በነባሪ ቦታ ላይ ያስቀምጡ፦
# - Linux፦ ~/.config/stonktop/config.toml
# - macOS፦ ~/Library/Application Support/stonktop/config.toml
# - Windows፦ %APPDATA%\stonktop\config.toml
```

## የትዕዛዝ መስመር አማራጮች

| አማራጭ | አጭር | መግለጫ |
|--------|------|--------|
| `--symbols` | `-s` | በነጠላ ሰረዝ የተለዩ የሚከታተሏቸው ምልክቶች ዝርዝር |
| `--delay` | `-d` | የማደሻ መዘግየት በሰከንድ (ነባሪ፦ 5) |
| `--iterations` | `-n` | የድግግሞሽ ብዛት (0 = ያልተገደበ) |
| `--batch` | `-b` | የባች ሁነታ - መስተጋብራዊ ያልሆነ ውጤት |
| `--secure` | `-S` | ደህንነቱ የተጠበቀ ሁነታ - መስተጋብራዊ ትዕዛዞችን ያሰናክላል |
| `--config` | `-c` | የማዋቀሪያ ፋይል ዱካ |
| `--sort` | `-o` | የመጀመሪያ የመደርደሪያ መስክ |
| `--reverse` | `-r` | የመደርደሪያ ቅደም ተከተልን ገልብጥ |
| `--holdings` | `-H` | የንብረት ይዞታ/ፖርትፎሊዮ እይታን አሳይ |
| `--currency` | | የማሳያ ምንዛሬ (ነባሪ፦ USD) |
| `--timeout` | | የ API ጊዜ ማብቂያ በሰከንድ (ነባሪ፦ 10) |
| `--format` | | ለባች ሁነታ የውጤት ቅርጸት፦ table፣ json፣ csv |
| `--color` | | የቀለም ውጤት፦ auto፣ always፣ never |
| `--verbose` | `-v` | ዝርዝር ውጤት |
| `--init` | | የናሙና የማዋቀሪያ ፋይል ፍጠር |
| `--force` | | ያለውን የማዋቀሪያ ፋይል በግዴታ ተካ (ከ `--init` ጋር) |
| `--lang` | | የማሳያ ቋንቋ (`STONKTOP_LANG`፣ ነባሪ፦ ከ `LC_ALL`/`LC_MESSAGES`/`LANG` በራስ-ሰር) |
| `--help` | | የእገዛ መልዕክት አሳይ |
| `--version` | `-V` | ስሪት አሳይ |

## የሚደገፉ ቋንቋዎች

Stonktop የማሳያ ቋንቋውን ከ `STONKTOP_LANG`፣ `LC_ALL`፣ `LC_MESSAGES` እና `LANG` በራስ-ሰር ያውቃል፣ ወይም `--lang <LANG>` ይቀበላል። ሁሉም 45 የሚደገፉ ቋንቋዎችና ክልሎች ሙሉ የ CLI እና TUI ትርጉሞችን (ለአረብኛ፣ ፋርሲ፣ ዕብራይስጥ እና ኡርዱ ከቀኝ ወደ ግራ አሰላለፍን ጨምሮ) እንዲሁም የተተረጎመ ሰነድ ያካትታሉ፦

| ኮድ | ቋንቋ | README | ኮድ | ቋንቋ | README | ኮድ | ቋንቋ | README |
|-----|------|--------|-----|------|--------|-----|------|--------|
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

## የቁልፍ ሰሌዳ አቋራጮች

| ቁልፍ | ተግባር |
|------|-------|
| `q`, `Esc` | ውጣ |
| `h`, `?` | እገዛን አብራ/አጥፋ |
| `↑`, `k` | ወደ ላይ ውጣ |
| `↓`, `j` | ወደ ታች ውረድ |
| `g`, `Home` | ወደ መጀመሪያው ሂድ |
| `G`, `End` | ወደ መጨረሻው ሂድ |
| `PgUp` | አንድ ገጽ ወደ ላይ |
| `PgDn` | አንድ ገጽ ወደ ታች |
| `s` | የመደርደሪያ መስኮችን ቀያይር |
| `r` | የመደርደሪያ ቅደም ተከተልን ገልብጥ |
| `1-7` | በዓምድ ደርድር |
| `Enter` | ለተመረጠው ምልክት የዝርዝር መስኮት አሳይ |
| `/` | ምልክቶችን ፈልግ/አጣራ |
| `a` | ወደ ክትትል ዝርዝር ምልክት ጨምር |
| `d` | የተመረጠውን ምልክት አስወግድ |
| `S` | የስፓርክላይን ገበታ የጎን አሞሌን አብራ/አጥፋ |
| `e` | የክትትል ዝርዝሩን ወደ JSON ላክ |
| `H` | የንብረት ይዞታ እይታን አብራ/አጥፋ |
| `f` | መሠረታዊ የፋይናንስ መረጃዎችን አብራ/አጥፋ |
| `Space`, `R` | በግዴታ አድስ |
| `Tab` | የምልክት ቡድኖችን ቀያይር |

## ማዋቀሪያ

የ TOML ማዋቀሪያ ፋይል ይፍጠሩ፦

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

# የፖርትፎሊዮ ይዞታዎች (አማራጭ)
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

# የምልክት ቡድኖች
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# የብዙ ምንዛሬ ማሳያ (አማራጭ)
[currency]
display = "EUR"
convert = true

# የዋጋ ማንቂያዎች (አማራጭ)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# ብጁ የክሪፕቶ አቋራጮች (አማራጭ)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## የምልክት ቅርጸቶች

| ቅርጸት | መግለጫ | ምሳሌ |
|-------|--------|------|
| አክሲዮን | መደበኛ የአክሲዮን ምልክት | `AAPL`፣ `GOOGL` |
| ክሪፕቶ | ከ USD ቅጥያ ጋር | `BTC-USD`፣ `ETH-USD` |
| የክሪፕቶ አጭር ቅጽ | በራስ-ሰር ወደ -USD ይዘረጋል | `BTC.X` -> `BTC-USD` |
| ራስ-ሰር ክሪፕቶ | የተለመዱ የክሪፕቶ ምንዛሬዎች | `BTC` -> `BTC-USD` |

## የውሂብ ምንጭ

Stonktop የቅጽበታዊ ዋጋ ተመኖችን ለማምጣት የ Yahoo Finance API ይጠቀማል። ምንም የ API ቁልፍ አያስፈልግም።

## ልማት

### መገንባት

```bash
# የማረሚያ ግንባታ
cargo build

# የልቀት ግንባታ
cargo build --release
```

### ሙከራ

```bash
# ሁሉንም ሙከራዎች አሂድ
cargo test

# ሙከራዎችን ከውጤት ጋር አሂድ
cargo test -- --nocapture
```

### የኮድ ጥራት ፍተሻ

```bash
# የኮድ ጥራት ችግሮችን ፈትሽ
cargo clippy --all-targets --all-features

# ራስ-ሰር ማስተካከያዎችን ተግብር
cargo clippy --fix --allow-dirty
```

### የኮድ ቅርጸት

```bash
# ቅርጸቱን ፈትሽ
cargo fmt -- --check

# ቅርጸቱን ተግብር
cargo fmt
```

## አስተዋጽኦ ማድረግ

አስተዋጽኦዎች በደስታ ይቀበላሉ! እባክዎ የለውጥ ጥያቄ ለማቅረብ አያመንቱ።

1. ማከማቻውን ፎርክ ያድርጉ
2. የባህሪ ቅርንጫፍዎን ይፍጠሩ (`git checkout -b feature/amazing-feature`)
3. ለውጦችዎን ያስቀምጡ (`git commit -m 'Add amazing feature'`)
4. ቅርንጫፉን ወደ ርቀት ማከማቻ ይላኩ (`git push origin feature/amazing-feature`)
5. የለውጥ ጥያቄ ይክፈቱ

## ፈቃድ

ይህ ፕሮጀክት በ MIT ፈቃድ ስር ፈቃድ የተሰጠው ነው — ለዝርዝሮች የ[LICENSE](../../../LICENSE) ፋይልን ይመልከቱ።

## ምስጋና

- ከ [ticker](https://github.com/achannarasappa/ticker) በተገኘ መነሳሳት
- በ [ratatui](https://github.com/ratatui-org/ratatui) የተገነባ
- ውሂብ ከ [Yahoo Finance](https://finance.yahoo.com/)
