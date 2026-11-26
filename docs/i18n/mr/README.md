# Stonktop

> [English](../../../README.md)

समभाग (शेअर्स) आणि क्रिप्टोकरन्सीच्या किमती रिअल-टाइममध्ये पाहण्यासाठी `top` सारखा टर्मिनल वापरकर्ता संवादपट (TUI).

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## वैशिष्ट्ये

- **रिअल-टाइम किंमत निरीक्षण**: समभाग आणि क्रिप्टोकरन्सीचे थेट दर
- **`top`-सारखा संवादपट**: `top`/`htop` सारखी परिचित कीबोर्ड नियंत्रणे
- **स्पार्कलाइन आलेख**: बाजूच्या पट्टीत (साइडबारमध्ये) किमतीच्या प्रवाहाचे थेट दृश्यमान चित्रण
- **पोर्टफोलिओ मागोवा**: नफा/तोटा (P/L) गणनेसह तुमच्या गुंतवणुकीवर लक्ष ठेवा
- **शोध आणि गाळणी**: चिन्हे आणि नावांमध्ये अक्षरांच्या आकारभेदाशिवाय (केस-इनसेन्सिटिव्ह) शोध
- **किंमत सूचना**: किमती निर्धारित मर्यादा ओलांडल्यावर डेस्कटॉप सूचना
- **निर्यात/आयात**: निरीक्षण यादी (वॉचलिस्ट) JSON किंवा CSV स्वरूपात जतन करा आणि उघडा
- **बहु-चलन समर्थन**: थेट विनिमय दरांसह कोणत्याही चलनात किमती दाखवा
- **विविध क्रमवारी पर्याय**: चिन्ह, किंमत, बदल, उलाढाल (व्हॉल्यूम), बाजार भांडवल (मार्केट कॅप) नुसार क्रमवारी लावा
- **बॅच मोड**: स्क्रिप्टिंगसाठी परस्परसंवादरहित आउटपुट (`top -b` प्रमाणे)
- **TOML संरचना**: संपादित करण्यास सोप्या संरचना फाइल्स
- **रंग संकेतन**: वाढीसाठी हिरवा रंग, घसरणीसाठी लाल रंग
- **क्रिप्टो लघुनामे**: `BTC-USD` साठी `BTC.X` किंवा फक्त `BTC` वापरा

## स्क्रीनशॉट्स

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

## पूर्वआवश्यकता

- **Rust**: आवृत्ती 1.89 किंवा त्यापुढील (किमान समर्थित आवृत्ती — MSRV)
- **Cargo**: Rust स्थापनेसोबत समाविष्ट

## स्थापना

### crates.io वरून (शिफारस केलेले)

```bash
cargo install stonktop
```

### cargo-binstall वापरून

तुमच्या प्रणालीवर [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) स्थापित असल्यास, तुम्ही पूर्व-निर्मित बायनरी थेट स्थापित करू शकता:

```bash
cargo binstall stonktop
```

### पूर्व-निर्मित बायनरी

[Releases](https://github.com/somethingwithproof/stonktop/releases) पृष्ठावरून पूर्व-निर्मित बायनरी डाउनलोड करा.

उपलब्ध प्लॅटफॉर्म:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### स्रोत कोडवरून

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## वापर

### मूलभूत वापर

```bash
# विशिष्ट चिन्हांवर लक्ष ठेवा
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# क्रिप्टो लघुनामांसह
stonktop -s AAPL,BTC,ETH,SOL
```

### top-सारखे पर्याय

```bash
# अद्यतन अंतराल ठरवा (top -d प्रमाणे)
stonktop -s AAPL,GOOGL -d 10

# N फेऱ्या चालवून बाहेर पडा (top -n प्रमाणे)
stonktop -s AAPL -n 5

# स्क्रिप्टिंगसाठी बॅच मोड (top -b प्रमाणे)
stonktop -s AAPL,GOOGL -b

# सुरक्षित मोड - परस्परसंवादी आदेश निष्क्रिय करा
stonktop -s AAPL -S
```

### क्रमवारी पर्याय

```bash
# किमतीनुसार क्रमवारी लावा (उतरता क्रम)
stonktop -s AAPL,GOOGL -o price

# चिन्हानुसार क्रमवारी लावा (चढता क्रम)
stonktop -s AAPL,GOOGL -o symbol -r

# उपलब्ध क्रमवारी क्षेत्रे: symbol, name, price, change, change-percent, volume, market-cap
```

### संरचना फाइल

```bash
# विशिष्ट संरचना फाइल वापरा
stonktop -c ~/.config/stonktop/custom.toml

# किंवा संरचना फाइल मूलभूत स्थानावर ठेवा:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## कमांड-लाइन पर्याय

| पर्याय | संक्षेप | वर्णन |
|--------|---------|-------|
| `--symbols` | `-s` | लक्ष ठेवण्यासाठी स्वल्पविरामाने विभक्त केलेल्या चिन्हांची यादी |
| `--delay` | `-d` | अद्यतन अंतराल सेकंदांमध्ये (मूलभूत: 5) |
| `--iterations` | `-n` | फेऱ्यांची संख्या (0 = अमर्याद) |
| `--batch` | `-b` | बॅच मोड — परस्परसंवादरहित आउटपुट |
| `--secure` | `-S` | सुरक्षित मोड — परस्परसंवादी आदेश निष्क्रिय करा |
| `--config` | `-c` | संरचना फाइलचा मार्ग |
| `--sort` | `-o` | प्रारंभिक क्रमवारी क्षेत्र |
| `--reverse` | `-r` | क्रमवारी उलट करा |
| `--holdings` | `-H` | होल्डिंग्ज/पोर्टफोलिओ दृश्य दाखवा |
| `--currency` | | प्रदर्शनाचे चलन (मूलभूत: USD) |
| `--timeout` | | API कालमर्यादा सेकंदांमध्ये (मूलभूत: 10) |
| `--format` | | बॅच मोडसाठी आउटपुट स्वरूप: `table`, `json`, `csv` |
| `--color` | | रंगीत आउटपुट: `auto`, `always`, `never` |
| `--verbose` | `-v` | सविस्तर आउटपुट |
| `--init` | | नमुना संरचना फाइल तयार करा |
| `--force` | | अस्तित्वात असलेली संरचना फाइल जबरदस्तीने पुनर्लेखित करा (`--init` सोबत) |
| `--lang` | | प्रदर्शन भाषा (`STONKTOP_LANG`, मूलभूत: `LC_ALL`/`LC_MESSAGES`/`LANG` मधून स्वयंचलित) |
| `--help` | | मदत संदेश दाखवा |
| `--version` | `-V` | आवृत्ती क्रमांक दाखवा |

## समर्थित भाषा

Stonktop हे `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` आणि `LANG` पर्यावरण चलांवरून प्रदर्शन भाषा ओळखते किंवा `--lang <LANG>` पर्याय स्वीकारते. सर्व ४८ समर्थित लोकेल्समध्ये संपूर्ण CLI आणि TUI भाषांतरे (अरबी, फारसी, हिब्रू आणि उर्दूसाठी उजवीकडून डावीकडे संरेखनासह) आणि स्थानिकीकृत दस्तऐवजीकरण समाविष्ट आहे:

| सांकेतांक | भाषा | README | सांकेतांक | भाषा | README | सांकेतांक | भाषा | README |
|-----------|------|--------|-----------|------|--------|-----------|------|--------|
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

| की | कृती |
|----|------|
| `q`, `Esc` | बाहेर पडा |
| `h`, `?` | मदत दृश्य चालू/बंद करा |
| `↑`, `k` | वर जा |
| `↓`, `j` | खाली जा |
| `g`, `Home` | यादीच्या शीर्षस्थानी जा |
| `G`, `End` | यादीच्या तळाशी जा |
| `PgUp` | एक पान वर जा |
| `PgDn` | एक पान खाली जा |
| `s` | क्रमवारीचे क्षेत्र बदला |
| `r` | क्रमवारीचा क्रम उलट करा |
| `1-7` | स्तंभानुसार क्रमवारी लावा |
| `Enter` | निवडलेल्या चिन्हाची सविस्तर माहिती पॉप-अपमध्ये दाखवा |
| `/` | चिन्हे शोधा/गाळा |
| `a` | निरीक्षण यादीत नवीन चिन्ह जोडा |
| `d` | निवडलेले चिन्ह काढा |
| `S` | स्पार्कलाइन आलेख बाजूची पट्टी चालू/बंद करा |
| `e` | निरीक्षण यादी JSON मध्ये निर्यात करा |
| `H` | होल्डिंग्ज दृश्य चालू/बंद करा |
| `f` | मूलभूत आर्थिक माहिती (फंडामेंटल्स) चालू/बंद करा |
| `Space`, `R` | तात्काळ अद्यतनित करा |
| `Tab` | चिन्ह गटांमध्ये फेरबदल करा |

## संरचना

TOML संरचना फाइल तयार करा:

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

# पोर्टफोलिओ होल्डिंग्ज (ऐच्छिक)
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

# चिन्ह गट
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# बहु-चलन प्रदर्शन (ऐच्छिक)
[currency]
display = "EUR"
convert = true

# किंमत सूचना (ऐच्छिक)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# सानुकूल क्रिप्टो लघुनामे (ऐच्छिक)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## चिन्ह स्वरूपे

| स्वरूप | वर्णन | उदाहरण |
|-------|-------|--------|
| समभाग | प्रमाणित टिकर | `AAPL`, `GOOGL` |
| क्रिप्टो | USD प्रत्ययासह | `BTC-USD`, `ETH-USD` |
| क्रिप्टो लघुस्वरूप | `-USD` मध्ये विस्तारित होते | `BTC.X` -> `BTC-USD` |
| स्वयंचलित क्रिप्टो | प्रचलित क्रिप्टोकरन्सी | `BTC` -> `BTC-USD` |

## माहिती स्रोत

Stonktop थेट दर मिळवण्यासाठी Yahoo Finance API वापरते. यासाठी कोणत्याही API कीची आवश्यकता नाही.

## विकास

### बांधणी (बिल्ड)

```bash
# डीबग बिल्ड
cargo build

# रिलीज बिल्ड
cargo build --release
```

### चाचणी

```bash
# सर्व चाचण्या चालवा
cargo test

# आउटपुटसह चाचण्या चालवा
cargo test -- --nocapture
```

### लिंट तपासणी

```bash
# लिंटिंग समस्या तपासा
cargo clippy --all-targets --all-features

# स्वयंचलित दुरुस्त्या लागू करा
cargo clippy --fix --allow-dirty
```

### स्वरूपण

```bash
# कोड स्वरूपण तपासा
cargo fmt -- --check

# कोड स्वरूपण लागू करा
cargo fmt
```

## योगदान

तुमच्या योगदानाचे स्वागत आहे! कृपया बिनदिक्कत पुल रिक्वेस्ट (Pull Request) सादर करा.

1. रिपॉझिटरी फोर्क करा
2. तुमची वैशिष्ट्य शाखा तयार करा (`git checkout -b feature/amazing-feature`)
3. तुमचे बदल कमिट करा (`git commit -m 'Add amazing feature'`)
4. शाखेवर पुश करा (`git push origin feature/amazing-feature`)
5. पुल रिक्वेस्ट (Pull Request) उघडा

## परवाना

हा प्रकल्प MIT परवान्याअंतर्गत परवानाकृत आहे — तपशीलांसाठी [LICENSE](../../../LICENSE) फाइल पहा.

## आभार

- [ticker](https://github.com/achannarasappa/ticker) पासून प्रेरित
- [ratatui](https://github.com/ratatui-org/ratatui) वापरून निर्मित
- माहिती स्रोत: [Yahoo Finance](https://finance.yahoo.com/)
