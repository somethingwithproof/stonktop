# Stonktop

> [English](../../../README.md)

حقیقی وقت میں اسٹاک اور کرپٹو کرنسی کی قیمتوں کی نگرانی کے لیے `top` طرز کا ٹرمینل صارف انٹرفیس (TUI)۔

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## خصوصیات

- **حقیقی وقت میں قیمتوں کی نگرانی**: اسٹاک اور کرپٹو کرنسیوں کے براہِ راست نرخ
- **`top` طرز کا انٹرفیس**: `top`/`htop` جیسے مانوس کی بورڈ کنٹرولز
- **اسپارک لائن چارٹس**: سائیڈ بار میں قیمت کے رجحان کا فوری بصری مظاہرہ
- **پورٹ فولیو ٹریکنگ**: نفع و نقصان (P/L) کے حساب کے ساتھ اپنی ہولڈنگز کی نگرانی کریں
- **تلاش اور فلٹرنگ**: علامتوں اور ناموں میں بڑے یا چھوٹے حروف کی تخصیص کے بغیر تلاش
- **قیمت کے انتباہات**: قیمتوں کے مقررہ حدود سے تجاوز کرنے پر ڈیسک ٹاپ اطلاعات
- **برآمد/درآمد**: واچ لسٹس کو JSON یا CSV فارمیٹ میں محفوظ اور لوڈ کریں
- **کثیر کرنسی سہولت**: براہِ راست تبادلہ نرخوں کے ساتھ کسی بھی کرنسی میں قیمتیں دکھائیں
- **متعدد ترتیب اختیارات**: علامت، قیمت، تبدیلی، تجارتی حجم اور مارکیٹ کیپ کے لحاظ سے ترتیب دیں
- **بیچ موڈ**: اسکرپٹنگ کے لیے غیر تعاملی آؤٹ پٹ (`top -b` کی طرح)
- **TOML ترتیب**: آسانی سے ترمیم کی جانے والی ترتیبی فائلیں
- **رنگوں کی کوڈنگ**: اضافے کے لیے سبز، کمی کے لیے سرخ
- **کرپٹو شارٹ کٹس**: `BTC-USD` کے لیے `BTC.X` یا صرف `BTC` استعمال کریں

## اسکرین شاٹس

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

## پیشگی شرائط

- **Rust**: ورژن 1.89 یا اس سے جدید تر (کم از کم معاون ورژن — MSRV)
- **Cargo**: Rust کی تنصیب کے ساتھ شامل

## تنصیب

### crates.io سے (تجویز کردہ)

```bash
cargo install stonktop
```

### cargo-binstall کے ذریعے

اگر آپ کے سسٹم میں [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) نصب ہے تو آپ پہلے سے تیار کردہ بائنریز نصب کر سکتے ہیں:

```bash
cargo binstall stonktop
```

### پہلے سے تیار کردہ بائنریز

[Releases](https://github.com/somethingwithproof/stonktop/releases) کے صفحے سے پہلے سے تیار کردہ بائنریز ڈاؤن لوڈ کریں۔

دستیاب پلیٹ فارمز:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### سورس کوڈ سے

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## استعمال

### بنیادی استعمال

```bash
# مخصوص علامتوں کی نگرانی کریں
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# کرپٹو شارٹ کٹس کے ساتھ
stonktop -s AAPL,BTC,ETH,SOL
```

### top طرز کے اختیارات

```bash
# ریفریش وقفہ مقرر کریں (top -d کی طرح)
stonktop -s AAPL,GOOGL -d 10

# N تکرار چلا کر باہر نکلیں (top -n کی طرح)
stonktop -s AAPL -n 5

# اسکرپٹنگ کے لیے بیچ موڈ (top -b کی طرح)
stonktop -s AAPL,GOOGL -b

# محفوظ موڈ - تعاملی کمانڈز کو غیر فعال کریں
stonktop -s AAPL -S
```

### ترتیب کے اختیارات

```bash
# قیمت کے لحاظ سے ترتیب دیں (نزولی ترتیب)
stonktop -s AAPL,GOOGL -o price

# علامت کے لحاظ سے ترتیب دیں (صعودی ترتیب)
stonktop -s AAPL,GOOGL -o symbol -r

# دستیاب ترتیبی فیلڈز: symbol, name, price, change, change-percent, volume, market-cap
```

### ترتیبی فائل

```bash
# مخصوص ترتیبی فائل استعمال کریں
stonktop -c ~/.config/stonktop/custom.toml

# یا ترتیبی فائل کو طے شدہ مقام پر رکھیں:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## کمانڈ لائن اختیارات

| اختیار | مختصر | تفصیل |
|--------|-------|-------|
| `--symbols` | `-s` | نگرانی کے لیے کوما سے الگ شدہ علامتوں کی فہرست |
| `--delay` | `-d` | سیکنڈوں میں ریفریش وقفہ (طے شدہ: 5) |
| `--iterations` | `-n` | تکرار کی تعداد (0 = لامحدود) |
| `--batch` | `-b` | بیچ موڈ — غیر تعاملی آؤٹ پٹ |
| `--secure` | `-S` | محفوظ موڈ — تعاملی کمانڈز کو غیر فعال کریں |
| `--config` | `-c` | ترتیبی فائل کا راستہ |
| `--sort` | `-o` | ابتدائی ترتیبی فیلڈ |
| `--reverse` | `-r` | ترتیب کو الٹ دیں |
| `--holdings` | `-H` | ہولڈنگز/پورٹ فولیو منظر دکھائیں |
| `--currency` | | نمائشی کرنسی (طے شدہ: USD) |
| `--timeout` | | سیکنڈوں میں API وقت کی حد (طے شدہ: 10) |
| `--format` | | بیچ موڈ کے لیے آؤٹ پٹ فارمیٹ: `table`, `json`, `csv` |
| `--color` | | رنگین آؤٹ پٹ: `auto`, `always`, `never` |
| `--verbose` | `-v` | تفصیلی آؤٹ پٹ |
| `--init` | | نمونہ ترتیبی فائل تیار کریں |
| `--force` | | موجودہ ترتیب کو جبراً اوور رائٹ کریں (`--init` کے ساتھ) |
| `--lang` | | نمائشی زبان (`STONKTOP_LANG`، طے شدہ: `LC_ALL`/`LC_MESSAGES`/`LANG` سے خودکار) |
| `--help` | | مدد کا پیغام دکھائیں |
| `--version` | `-V` | ورژن دکھائیں |

## معاون زبانیں

Stonktop نمائشی زبان کا تعین `STONKTOP_LANG`، `LC_ALL`، `LC_MESSAGES` اور `LANG` سے کرتا ہے، یا `--lang <LANG>` قبول کرتا ہے۔ تمام 48 معاون لوکیلز میں مکمل CLI اور TUI تراجم (عربی، فارسی، عبرانی اور اردو کے لیے دائیں سے بائیں سیدھ سمیت) اور مقامی دستاویزات شامل ہیں:

| کوڈ | زبان | README | کوڈ | زبان | README | کوڈ | زبان | README |
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

## کی بورڈ شارٹ کٹس

| کلید | عمل |
|------|-----|
| `q`, `Esc` | باہر نکلیں |
| `h`, `?` | مدد کا منظر ٹوگل کریں |
| `↑`, `k` | اوپر جائیں |
| `↓`, `j` | نیچے جائیں |
| `g`, `Home` | سب سے اوپر جائیں |
| `G`, `End` | سب سے نیچے جائیں |
| `PgUp` | ایک صفحہ اوپر |
| `PgDn` | ایک صفحہ نیچے |
| `s` | ترتیبی فیلڈ تبدیل کریں |
| `r` | ترتیب کو الٹ دیں |
| `1-7` | کالم کے لحاظ سے ترتیب دیں |
| `Enter` | منتخب علامت کی تفصیلی پاپ اپ دکھائیں |
| `/` | علامتیں تلاش/فلٹر کریں |
| `a` | واچ لسٹ میں علامت شامل کریں |
| `d` | منتخب علامت حذف کریں |
| `S` | اسپارک لائن چارٹس سائیڈ بار ٹوگل کریں |
| `e` | واچ لسٹ کو JSON میں برآمد کریں |
| `H` | ہولڈنگز منظر ٹوگل کریں |
| `f` | بنیادی مالی معلومات (فنڈامینٹلز) ٹوگل کریں |
| `Space`, `R` | فوری ریفریش کریں |
| `Tab` | علامتوں کے گروپس کے درمیان تبدیل کریں |

## ترتیب

ایک TOML ترتیبی فائل تیار کریں:

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

# پورٹ فولیو ہولڈنگز (اختیاری)
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

# علامتوں کے گروپس
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# کثیر کرنسی نمائش (اختیاری)
[currency]
display = "EUR"
convert = true

# قیمت کے انتباہات (اختیاری)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# حسبِ ضرورت کرپٹو شارٹ کٹس (اختیاری)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## علامت کے فارمیٹس

| فارمیٹ | تفصیل | مثال |
|--------|-------|------|
| اسٹاک | معیاری ٹکر | `AAPL`, `GOOGL` |
| کرپٹو | USD لاحقے کے ساتھ | `BTC-USD`, `ETH-USD` |
| کرپٹو مختصر شکل | `-USD` میں پھیلتا ہے | `BTC.X` -> `BTC-USD` |
| کرپٹو خودکار | عام کرپٹو کرنسیاں | `BTC` -> `BTC-USD` |

## ڈیٹا ماخذ

Stonktop حقیقی وقت کے نرخ حاصل کرنے کے لیے Yahoo Finance API استعمال کرتا ہے۔ کسی API کلید کی ضرورت نہیں ہے۔

## ترقی

### بلڈ

```bash
# ڈی بگ بلڈ
cargo build

# ریلیز بلڈ
cargo build --release
```

### ٹیسٹ

```bash
# تمام ٹیسٹ چلائیں
cargo test

# آؤٹ پٹ کے ساتھ ٹیسٹ چلائیں
cargo test -- --nocapture
```

### لنٹ

```bash
# لنٹنگ مسائل کی جانچ کریں
cargo clippy --all-targets --all-features

# خودکار اصلاحات لاگو کریں
cargo clippy --fix --allow-dirty
```

### فارمیٹ

```bash
# فارمیٹنگ کی جانچ کریں
cargo fmt -- --check

# فارمیٹنگ لاگو کریں
cargo fmt
```

## تعاون

تعاون کا خیرمقدم ہے! بلا جھجک Pull Request جمع کروائیں۔

1. ریپوزٹری کو فورک کریں
2. اپنی فیچر برانچ بنائیں (`git checkout -b feature/amazing-feature`)
3. اپنی تبدیلیاں کمٹ کریں (`git commit -m 'Add amazing feature'`)
4. برانچ پر پش کریں (`git push origin feature/amazing-feature`)
5. ایک Pull Request کھولیں

## لائسنس

یہ منصوبہ MIT لائسنس کے تحت جاری کیا گیا ہے — تفصیلات کے لیے [LICENSE](../../../LICENSE) فائل ملاحظہ کریں۔

## اظہارِ تشکر

- [ticker](https://github.com/achannarasappa/ticker) سے متاثر
- [ratatui](https://github.com/ratatui-org/ratatui) کی مدد سے تیار کردہ
- ڈیٹا ماخذ: [Yahoo Finance](https://finance.yahoo.com/)
