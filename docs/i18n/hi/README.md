# Stonktop

> [English](../../../README.md)

वास्तविक समय में शेयर और क्रिप्टोकरेंसी की कीमतों की निगरानी के लिए `top` जैसा टर्मिनल यूज़र इंटरफ़ेस (TUI)।

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## प्रमुख विशेषताएँ

- **वास्तविक समय मूल्य निगरानी**: शेयरों और क्रिप्टोकरेंसी के लाइव भाव
- **`top` जैसा इंटरफ़ेस**: `top`/`htop` के समान परिचित कीबोर्ड नियंत्रण
- **स्पार्कलाइन चार्ट**: साइडबार में इनलाइन मूल्य प्रवृत्ति का दृश्यांकन
- **पोर्टफोलियो ट्रैकिंग**: लाभ/हानि (P/L) गणना के साथ अपनी होल्डिंग्स की निगरानी करें
- **खोज और फ़िल्टर**: प्रतीकों और नामों में केस-इनसेंसिटिव खोज
- **मूल्य अलर्ट**: कीमतें निर्धारित सीमा पार करने पर डेस्कटॉप सूचनाएँ
- **निर्यात/आयात (Export/Import)**: वॉचलिस्ट को JSON या CSV के रूप में सहेजें और लोड करें
- **बहु-मुद्रा समर्थन**: लाइव विनिमय दरों के साथ किसी भी मुद्रा में कीमतें प्रदर्शित करें
- **एकाधिक क्रमितकरण विकल्प**: प्रतीक, मूल्य, परिवर्तन, वॉल्यूम या बाज़ार पूँजीकरण के आधार पर क्रमबद्ध करें
- **बैच मोड**: स्क्रिप्टिंग के लिए गैर-इंटरैक्टिव आउटपुट (`top -b` की तरह)
- **TOML कॉन्फ़िगरेशन**: संपादित करने में आसान कॉन्फ़िगरेशन फ़ाइलें
- **रंग-कोडित प्रदर्शन**: बढ़त के लिए हरा और गिरावट के लिए लाल रंग
- **क्रिप्टो शॉर्टकट**: `BTC-USD` के लिए `BTC.X` या केवल `BTC` का उपयोग करें

## स्क्रीनशॉट

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

## पूर्वापेक्षाएँ

- **Rust**: संस्करण 1.89 या उच्चतर (न्यूनतम समर्थित Rust संस्करण - MSRV)
- **Cargo**: Rust इंस्टॉलेशन के साथ सम्मिलित

## इंस्टॉलेशन

### crates.io से (अनुशंसित)

```bash
cargo install stonktop
```

### cargo-binstall का उपयोग करके

यदि आपके सिस्टम पर [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) इंस्टॉल है, तो आप पूर्व-निर्मित बाइनरी सीधे इंस्टॉल कर सकते हैं:

```bash
cargo binstall stonktop
```

### पूर्व-निर्मित बाइनरी

[Releases](https://github.com/somethingwithproof/stonktop/releases) पृष्ठ से पूर्व-निर्मित बाइनरी डाउनलोड करें।

उपलब्ध प्लेटफ़ॉर्म:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### स्रोत कोड से निर्माण

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## उपयोग

### बुनियादी उपयोग

```bash
# विशिष्ट प्रतीकों की निगरानी करें
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# क्रिप्टो शॉर्टकट के साथ
stonktop -s AAPL,BTC,ETH,SOL
```

### top जैसे विकल्प

```bash
# रीफ़्रेश अंतराल निर्धारित करें (top -d की तरह)
stonktop -s AAPL,GOOGL -d 10

# N पुनरावृत्तियाँ चलाकर बाहर निकलें (top -n की तरह)
stonktop -s AAPL -n 5

# स्क्रिप्टिंग के लिए बैच मोड (top -b की तरह)
stonktop -s AAPL,GOOGL -b

# सुरक्षित मोड - इंटरैक्टिव कमांड अक्षम करें
stonktop -s AAPL -S
```

### क्रमितकरण विकल्प

```bash
# मूल्य के अनुसार क्रमबद्ध करें (अवरोही)
stonktop -s AAPL,GOOGL -o price

# प्रतीक के अनुसार क्रमबद्ध करें (आरोही)
stonktop -s AAPL,GOOGL -o symbol -r

# उपलब्ध क्रमबद्ध फ़ील्ड: symbol, name, price, change, change-percent, volume, market-cap
```

### कॉन्फ़िगरेशन फ़ाइल

```bash
# किसी विशिष्ट कॉन्फ़िगरेशन फ़ाइल का उपयोग करें
stonktop -c ~/.config/stonktop/custom.toml

# या कॉन्फ़िगरेशन फ़ाइल को डिफ़ॉल्ट स्थान पर रखें:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## कमांड लाइन विकल्प

| विकल्प | संक्षिप्त | विवरण |
|--------|-----------|-------|
| `--symbols` | `-s` | निगरानी के लिए अल्पविराम से पृथक प्रतीकों की सूची |
| `--delay` | `-d` | सेकंड में रीफ़्रेश अंतराल (डिफ़ॉल्ट: 5) |
| `--iterations` | `-n` | पुनरावृत्तियों की संख्या (0 = अनंत) |
| `--batch` | `-b` | बैच मोड - गैर-इंटरैक्टिव आउटपुट |
| `--secure` | `-S` | सुरक्षित मोड - इंटरैक्टिव कमांड अक्षम करें |
| `--config` | `-c` | कॉन्फ़िगरेशन फ़ाइल का पथ |
| `--sort` | `-o` | प्रारंभिक क्रमबद्ध फ़ील्ड |
| `--reverse` | `-r` | क्रमबद्ध अनुक्रम उलटें |
| `--holdings` | `-H` | होल्डिंग्स/पोर्टफोलियो दृश्य दिखाएँ |
| `--currency` | | प्रदर्शन मुद्रा (डिफ़ॉल्ट: USD) |
| `--timeout` | | सेकंड में API टाइमआउट (डिफ़ॉल्ट: 10) |
| `--format` | | बैच मोड के लिए आउटपुट प्रारूप: table, json, csv |
| `--color` | | रंगीन आउटपुट: auto, always, never |
| `--verbose` | `-v` | विस्तृत आउटपुट |
| `--init` | | एक नमूना कॉन्फ़िगरेशन फ़ाइल उत्पन्न करें |
| `--force` | | मौजूदा कॉन्फ़िगरेशन को जबरन अधिलेखित करें (`--init` के साथ) |
| `--lang` | | प्रदर्शन भाषा (`STONKTOP_LANG`, डिफ़ॉल्ट: `LC_ALL`/`LC_MESSAGES`/`LANG` से स्वतः) |
| `--help` | | सहायता संदेश दिखाएँ |
| `--version` | `-V` | संस्करण दिखाएँ |

## समर्थित भाषाएँ

Stonktop `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` और `LANG` से प्रदर्शन भाषा का स्वतः पता लगाता है, या `--lang <LANG>` स्वीकार करता है। सभी 48 समर्थित लोकेल में पूर्ण CLI और TUI अनुवाद (अरबी, फ़ारसी, हिब्रू और उर्दू के लिए दाएँ-से-बाएँ संरेखण सहित) तथा स्थानीयकृत दस्तावेज़ शामिल हैं:

| कोड | भाषा | README | कोड | भाषा | README | कोड | भाषा | README |
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

## कीबोर्ड शॉर्टकट

| कुंजी | क्रिया |
|-------|--------|
| `q`, `Esc` | बाहर निकलें |
| `h`, `?` | सहायता टॉगल करें |
| `↑`, `k` | ऊपर जाएँ |
| `↓`, `j` | नीचे जाएँ |
| `g`, `Home` | शीर्ष पर जाएँ |
| `G`, `End` | निचले भाग पर जाएँ |
| `PgUp` | एक पृष्ठ ऊपर |
| `PgDn` | एक पृष्ठ नीचे |
| `s` | क्रमबद्ध फ़ील्ड बदलें |
| `r` | क्रमबद्ध अनुक्रम उलटें |
| `1-7` | कॉलम के अनुसार क्रमबद्ध करें |
| `Enter` | चयनित प्रतीक के लिए विवरण पॉपअप दिखाएँ |
| `/` | प्रतीक खोजें/फ़िल्टर करें |
| `a` | वॉचलिस्ट में प्रतीक जोड़ें |
| `d` | चयनित प्रतीक हटाएँ |
| `S` | स्पार्कलाइन चार्ट साइडबार टॉगल करें |
| `e` | वॉचलिस्ट को JSON में निर्यात करें |
| `H` | होल्डिंग्स दृश्य टॉगल करें |
| `f` | मूलभूत संकेतक (Fundamentals) टॉगल करें |
| `Space`, `R` | जबरन रीफ़्रेश करें |
| `Tab` | प्रतीक समूहों के बीच बदलें |

## कॉन्फ़िगरेशन

एक TOML कॉन्फ़िगरेशन फ़ाइल बनाएँ:

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

# पोर्टफोलियो होल्डिंग्स (वैकल्पिक)
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

# प्रतीक समूह
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# बहु-मुद्रा प्रदर्शन (वैकल्पिक)
[currency]
display = "EUR"
convert = true

# मूल्य अलर्ट (वैकल्पिक)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# कस्टम क्रिप्टो शॉर्टकट (वैकल्पिक)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## प्रतीक प्रारूप

| प्रारूप | विवरण | उदाहरण |
|--------|-------|--------|
| शेयर (Stock) | मानक टिकर | `AAPL`, `GOOGL` |
| क्रिप्टो | USD प्रत्यय के साथ | `BTC-USD`, `ETH-USD` |
| क्रिप्टो संक्षिप्त रूप | स्वतः -USD में विस्तृत होता है | `BTC.X` -> `BTC-USD` |
| क्रिप्टो स्वतः पहचान | सामान्य क्रिप्टोकरेंसी | `BTC` -> `BTC-USD` |

## डेटा स्रोत

Stonktop वास्तविक समय के भाव प्राप्त करने के लिए Yahoo Finance API का उपयोग करता है। किसी API कुंजी की आवश्यकता नहीं है।

## विकास

### निर्माण (Build)

```bash
# डीबग निर्माण
cargo build

# रिलीज़ निर्माण
cargo build --release
```

### परीक्षण (Test)

```bash
# सभी परीक्षण चलाएँ
cargo test

# आउटपुट के साथ परीक्षण चलाएँ
cargo test -- --nocapture
```

### लिंट (Lint)

```bash
# लिंटिंग समस्याओं की जाँच करें
cargo clippy --all-targets --all-features

# स्वचालित सुधार लागू करें
cargo clippy --fix --allow-dirty
```

### प्रारूपण (Format)

```bash
# प्रारूपण की जाँच करें
cargo fmt -- --check

# प्रारूपण लागू करें
cargo fmt
```

## योगदान

योगदान का स्वागत है! कृपया बेझिझक एक Pull Request प्रस्तुत करें।

1. रिपॉज़िटरी को फ़ॉर्क (Fork) करें
2. अपनी फ़ीचर शाखा बनाएँ (`git checkout -b feature/amazing-feature`)
3. अपने परिवर्तनों को कमिट करें (`git commit -m 'Add amazing feature'`)
4. शाखा को पुश करें (`git push origin feature/amazing-feature`)
5. एक Pull Request खोलें

## लाइसेंस

यह परियोजना MIT लाइसेंस के अंतर्गत लाइसेंस प्राप्त है — विवरण के लिए [LICENSE](../../../LICENSE) फ़ाइल देखें।

## आभार

- [ticker](https://github.com/achannarasappa/ticker) से प्रेरित
- [ratatui](https://github.com/ratatui-org/ratatui) के साथ निर्मित
- डेटा स्रोत: [Yahoo Finance](https://finance.yahoo.com/)
