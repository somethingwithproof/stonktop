# Stonktop

> [English](../../../README.md)

பங்கு மற்றும் கிரிப்டோகரன்சி விலைகளை நிகழ்நேரத்தில் கண்காணிப்பதற்கான `top` போன்ற முனைய பயனர் இடைமுகம் (TUI).

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## அம்சங்கள்

- **நிகழ்நேர விலை கண்காணிப்பு**: பங்குகள் மற்றும் கிரிப்டோகரன்சிகளுக்கான நேரடி விலைப்பட்டியல்கள்
- **`top` போன்ற இடைமுகம்**: `top`/`htop` போன்ற பழக்கமான விசைப்பலகை கட்டுப்பாடுகள்
- **ஸ்பார்க்லைன் வரைபடங்கள்**: பக்கவாட்டுப் பலகத்தில் உடனடி விலைப் போக்கு காட்சிப்படுத்தல்
- **முதலீட்டுத் தொகுப்பு கண்காணிப்பு**: இலாப/நட்ட (P/L) கணக்கீடுகளுடன் உங்கள் பங்கு இருப்பைக் கண்காணிக்கவும்
- **தேடல் மற்றும் வடிகட்டல்**: குறியீடுகள் மற்றும் பெயர்களில் எழுத்து வடிவ வேறுபாடற்ற தேடல்
- **விலை எச்சரிக்கைகள்**: விலைகள் அமைக்கப்பட்ட வரம்புகளைக் கடக்கும்போது கணினித் திரை அறிவிப்புகள்
- **ஏற்றுமதி/இறக்குமதி**: கண்காணிப்புப் பட்டியல்களை JSON அல்லது CSV வடிவில் சேமிக்கவும் ஏற்றவும்
- **பல நாணய ஆதரவு**: நேரடி மாற்று விகிதங்களுடன் எந்த நாணயத்திலும் விலைகளைக் காண்பிக்கவும்
- **பல வரிசைப்படுத்தல் விருப்பங்கள்**: குறியீடு, விலை, மாற்றம், பரிவர்த்தனை அளவு, சந்தை மூலதனம் ஆகியவற்றின் அடிப்படையில் வரிசைப்படுத்தல்
- **தொகுதி பயன்முறை**: ஸ்கிரிப்டிங்கிற்கான ஊடாடலற்ற வெளியீடு (`top -b` போன்றது)
- **TOML கட்டமைப்பு**: எளிதில் திருத்தக்கூடிய கட்டமைப்புக் கோப்புகள்
- **நிறக் குறியீட்டு முறை**: உயர்வுக்குப் பச்சை, சரிவுக்குச் சிவப்பு
- **கிரிப்டோ குறுக்குவழிகள்**: `BTC-USD` என்பதற்கு `BTC.X` அல்லது `BTC` எனப் பயன்படுத்தவும்

## திரைக்காட்சிகள்

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

## முன்நிபந்தனைகள்

- **Rust**: பதிப்பு 1.88 அல்லது அதற்கு மேற்பட்டது (குறைந்தபட்ச ஆதரிக்கப்படும் பதிப்பு — MSRV)
- **Cargo**: Rust நிறுவலுடன் உள்ளடக்கப்பட்டுள்ளது

## நிறுவல்

### crates.io இலிருந்து (பரிந்துரைக்கப்படுகிறது)

```bash
cargo install stonktop
```

### cargo-binstall பயன்படுத்துதல்

உங்கள் கணினியில் [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) நிறுவப்பட்டிருந்தால், முன்கூட்டியே உருவாக்கப்பட்ட இயங்கக்கூடிய கோப்புகளை நிறுவலாம்:

```bash
cargo binstall stonktop
```

### முன்கூட்டியே உருவாக்கப்பட்ட இயங்கக்கூடிய கோப்புகள்

[Releases](https://github.com/somethingwithproof/stonktop/releases) பக்கத்திலிருந்து முன்கூட்டியே உருவாக்கப்பட்ட இயங்கக்கூடிய கோப்புகளைப் பதிவிறக்கவும்.

கிடைக்கும் தளங்கள்:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### மூலக் குறியீட்டிலிருந்து

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## பயன்பாடு

### அடிப்படைப் பயன்பாடு

```bash
# குறிப்பிட்ட குறியீடுகளைக் கண்காணிக்கவும்
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# கிரிப்டோ குறுக்குவழிகளுடன்
stonktop -s AAPL,BTC,ETH,SOL
```

### top போன்ற விருப்பங்கள்

```bash
# புதுப்பிப்பு இடைவெளியை அமைக்கவும் (top -d போன்றது)
stonktop -s AAPL,GOOGL -d 10

# N சுழற்சிகளை இயக்கி வெளியேறவும் (top -n போன்றது)
stonktop -s AAPL -n 5

# ஸ்கிரிப்டிங்கிற்கான தொகுதி பயன்முறை (top -b போன்றது)
stonktop -s AAPL,GOOGL -b

# பாதுகாப்பான பயன்முறை - ஊடாடும் கட்டளைகளை முடக்கவும்
stonktop -s AAPL -S
```

### வரிசைப்படுத்தல் விருப்பங்கள்

```bash
# விலையின் அடிப்படையில் வரிசைப்படுத்தவும் (இறங்குவரிசை)
stonktop -s AAPL,GOOGL -o price

# குறியீட்டின் அடிப்படையில் வரிசைப்படுத்தவும் (ஏறுவரிசை)
stonktop -s AAPL,GOOGL -o symbol -r

# கிடைக்கும் வரிசைப்படுத்தல் புலங்கள்: symbol, name, price, change, change-percent, volume, market-cap
```

### கட்டமைப்புக் கோப்பு

```bash
# குறிப்பிட்ட கட்டமைப்புக் கோப்பைப் பயன்படுத்தவும்
stonktop -c ~/.config/stonktop/custom.toml

# அல்லது இயல்புநிலை இடத்தில் கட்டமைப்புக் கோப்பை வைக்கவும்:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## கட்டளை வரி விருப்பங்கள்

| விருப்பம் | சுருக்கம் | விளக்கம் |
|-----------|----------|----------|
| `--symbols` | `-s` | கண்காணிக்க வேண்டிய காற்புள்ளியால் பிரிக்கப்பட்ட குறியீடுகளின் பட்டியல் |
| `--delay` | `-d` | விநாடிகளில் புதுப்பிப்பு இடைவெளி (இயல்புநிலை: 5) |
| `--iterations` | `-n` | சுழற்சிகளின் எண்ணிக்கை (0 = முடிவற்றது) |
| `--batch` | `-b` | தொகுதி பயன்முறை — ஊடாடலற்ற வெளியீடு |
| `--secure` | `-S` | பாதுகாப்பான பயன்முறை — ஊடாடும் கட்டளைகளை முடக்கவும் |
| `--config` | `-c` | கட்டமைப்புக் கோப்பிற்கான பாதை |
| `--sort` | `-o` | தொடக்க வரிசைப்படுத்தல் புலம் |
| `--reverse` | `-r` | வரிசைப்படுத்தல் வரிசையைத் தலைகீழாக்கவும் |
| `--holdings` | `-H` | பங்கு இருப்பு/முதலீட்டுத் தொகுப்பு காட்சியைக் காண்பிக்கவும் |
| `--currency` | | காட்சி நாணயம் (இயல்புநிலை: USD) |
| `--timeout` | | விநாடிகளில் API காலாவதி நேரம் (இயல்புநிலை: 10) |
| `--format` | | தொகுதி பயன்முறைக்கான வெளியீட்டு வடிவம்: `table`, `json`, `csv` |
| `--color` | | நிற வெளியீடு: `auto`, `always`, `never` |
| `--verbose` | `-v` | விரிவான வெளியீடு |
| `--init` | | மாதிரி கட்டமைப்புக் கோப்பை உருவாக்கவும் |
| `--force` | | ஏற்கனவே உள்ள கட்டமைப்பை வலுக்கட்டாயமாக மேலெழுதவும் (`--init` உடன்) |
| `--lang` | | காட்சி மொழி (`STONKTOP_LANG`, இயல்புநிலை: `LC_ALL`/`LC_MESSAGES`/`LANG` இலிருந்து தானியங்கி) |
| `--help` | | உதவிச் செய்தியைக் காண்பிக்கவும் |
| `--version` | `-V` | பதிப்பைக் காண்பிக்கவும் |

## ஆதரிக்கப்படும் மொழிகள்

Stonktop காட்சி மொழியை `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, மற்றும் `LANG` ஆகியவற்றிலிருந்து கண்டறிகிறது, அல்லது `--lang <LANG>` விருப்பத்தை ஏற்கிறது. ஆதரிக்கப்படும் அனைத்து 45 மொழி அமைப்புகளிலும் முழுமையான CLI மற்றும் TUI மொழிபெயர்ப்புகளும் (அரபு, பாரசீகம், எபிரேயம் மற்றும் உருது மொழிகளுக்கான வலமிருந்து இடமாக சீரமைப்பு உட்பட) உள்ளூர்மயமாக்கப்பட்ட ஆவணங்களும் இடம்பெற்றுள்ளன:

| குறியீடு | மொழி | README | குறியீடு | மொழி | README | குறியீடு | மொழி | README |
|----------|------|--------|----------|------|--------|----------|------|--------|
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

## விசைப்பலகை குறுக்குவழிகள்

| விசை | செயல் |
|------|-------|
| `q`, `Esc` | வெளியேறு |
| `h`, `?` | உதவியை நிலைமாற்று |
| `↑`, `k` | மேலே நகர்த்து |
| `↓`, `j` | கீழே நகர்த்து |
| `g`, `Home` | மேல் பகுதிக்குச் செல் |
| `G`, `End` | கீழ் பகுதிக்குச் செல் |
| `PgUp` | ஒரு பக்கம் மேலே |
| `PgDn` | ஒரு பக்கம் கீழே |
| `s` | வரிசைப்படுத்தல் புலத்தை மாற்று |
| `r` | வரிசைப்படுத்தல் வரிசையைத் தலைகீழாக்கு |
| `1-7` | நெடுவரிசை வாரியாக வரிசைப்படுத்து |
| `Enter`, `d` | தேர்ந்தெடுக்கப்பட்ட குறியீட்டின் விரிவான சாளரத்தைக் காண்பி |
| `/` | குறியீடுகளைத் தேடு/வடிகட்டு |
| `a` | கண்காணிப்புப் பட்டியலில் குறியீட்டைச் சேர் |
| `x` | தேர்ந்தெடுக்கப்பட்ட குறியீட்டை நீக்கு |
| `S` | ஸ்பார்க்லைன் வரைபடப் பக்கவாட்டுப் பலகத்தை நிலைமாற்று |
| `e` | கண்காணிப்புப் பட்டியலை JSON வடிவில் ஏற்றுமதி செய் |
| `H` | பங்கு இருப்புக் காட்சியை நிலைமாற்று |
| `f` | அடிப்படை நிதி விவரங்களை நிலைமாற்று |
| `Space`, `R` | கட்டாயமாகப் புதுப்பி |
| `Tab` | குறியீட்டுக் குழுக்களிடையே மாற்று |

## கட்டமைப்பு

ஒரு TOML கட்டமைப்புக் கோப்பை உருவாக்கவும்:

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

# முதலீட்டுத் தொகுப்பு பங்கு இருப்பு (விருப்பத்தேர்வு)
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

# குறியீட்டுக் குழுக்கள்
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# பல நாணய காட்சி (விருப்பத்தேர்வு)
[currency]
display = "EUR"
convert = true

# விலை எச்சரிக்கைகள் (விருப்பத்தேர்வு)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# தனிப்பயன் கிரிப்டோ குறுக்குவழிகள் (விருப்பத்தேர்வு)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## குறியீட்டு வடிவங்கள்

| வடிவம் | விளக்கம் | எடுத்துக்காட்டு |
|--------|----------|----------------|
| பங்கு | நிலையான பங்குச் சந்தைக் குறியீடு | `AAPL`, `GOOGL` |
| கிரிப்டோ | USD பின்னொட்டுடன் | `BTC-USD`, `ETH-USD` |
| கிரிப்டோ சுருக்கம் | `-USD` ஆக விரிவடைகிறது | `BTC.X` -> `BTC-USD` |
| கிரிப்டோ தானியங்கி | பொதுவான கிரிப்டோகரன்சிகள் | `BTC` -> `BTC-USD` |

## தரவு மூலாதாரம்

நிகழ்நேர விலைப்பட்டியல்களைப் பெற Stonktop ஆனது Yahoo Finance API-ஐப் பயன்படுத்துகிறது. API விசை எதுவும் தேவையில்லை.

## மேம்பாடு

### உருவாக்கம்

```bash
# பிழைத்திருத்த உருவாக்கம்
cargo build

# வெளியீட்டு உருவாக்கம்
cargo build --release
```

### சோதனை

```bash
# அனைத்து சோதனைகளையும் இயக்கு
cargo test

# வெளியீட்டுடன் சோதனைகளை இயக்கு
cargo test -- --nocapture
```

### குறியீட்டுப் பகுப்பாய்வு (Lint)

```bash
# குறியீட்டுப் பகுப்பாய்வு சிக்கல்களைச் சரிபார்
cargo clippy --all-targets --all-features

# தானியங்கி திருத்தங்களைப் பயன்படுத்து
cargo clippy --fix --allow-dirty
```

### வடிவமைப்பு

```bash
# வடிவமைப்பைச் சரிபார்
cargo fmt -- --check

# வடிவமைப்பைப் பயன்படுத்து
cargo fmt
```

## பங்களிப்பு

பங்களிப்புகள் வரவேற்கப்படுகின்றன! தயங்காமல் Pull Request ஒன்றைச் சமர்ப்பிக்கவும்.

1. களஞ்சியத்தை Fork செய்யவும்
2. உங்கள் அம்சக் கிளையை உருவாக்கவும் (`git checkout -b feature/amazing-feature`)
3. உங்கள் மாற்றங்களை Commit செய்யவும் (`git commit -m 'Add amazing feature'`)
4. கிளைக்கு Push செய்யவும் (`git push origin feature/amazing-feature`)
5. ஒரு Pull Request-ஐத் திறக்கவும்

## உரிமம்

இந்தத் திட்டம் MIT உரிமத்தின் கீழ் உரிமம் பெற்றுள்ளது — விவரங்களுக்கு [LICENSE](../../../LICENSE) கோப்பைப் பார்க்கவும்.

## நன்றிகள்

- [ticker](https://github.com/achannarasappa/ticker) இலிருந்து உத்வேகம் பெற்றது
- [ratatui](https://github.com/ratatui-org/ratatui) கொண்டு உருவாக்கப்பட்டது
- தரவு வழங்கல்: [Yahoo Finance](https://finance.yahoo.com/)
