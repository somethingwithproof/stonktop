# Stonktop

> [English](../../../README.md)

واجهة طرفية (Terminal UI) على غرار أداة `top` لتتبع أثمنة الأسهم والعملات المشفرة بشكل آني ومباشر.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## المزايا الرئيسية

- **تتبع آني للأثمنة**: تسعيرات مباشرة للأسهم والعملات المشفرة في الوقت الحقيقي
- **واجهة على نمط `top`**: تحكم مألوف عبر لوحة المفاتيح يشبه `top`/`htop`
- **مبيانات مصغّرة (Sparkline)**: عرض مرئي فوري لمنحى تطور السعر داخل شريط جانبي
- **تتبع المحفظة المالية**: مراقبة الأصول المملوكة مع احتساب الأرباح والخسائر (P/L)
- **البحث والتصفية**: بحث غير حساس لحالة الأحرف عبر الرموز وتسميات القيم
- **تنبيهات الأثمنة**: إشعارات فورية على سطح المكتب عند تجاوز الأسعار للعتبات المحددة
- **التصدير والاستيراد**: حفظ واستيراد لوائح التتبع بصيغتي JSON أو CSV
- **تعدد العملات**: عرض الأثمنة بأي عملة مع التحويل الآني وفق أسعار الصرف الجارية
- **خيارات فرز متعددة**: ترتيب حسب الرمز، الثمن، التغير، حجم المعاملات، أو الرسملة السوقية
- **الوضع الدفعي (Batch Mode)**: إخراج غير تفاعلي مهيأ للسكربتات والأتمتة (مثل `top -b`)
- **تهيئة بصيغة TOML**: ملفات إعداد واضحة وسهلة التحرير
- **ترميز لوني واضح**: الأخضر للارتفاعات والأحمر للتراجعات
- **اختصارات العملات المشفرة**: استعمل `BTC.X` أو فقط `BTC` للدلالة على `BTC-USD`

## لقطات من الواجهة

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

## المتطلبات القبلية

- **Rust**: الإصدار 1.88 أو ما يفوق (الحد الأدنى المدعوم MSRV)
- **Cargo**: مدمج ضمن حزمة تثبيت Rust

## التثبيت

### عبر crates.io (موصى به)

```bash
cargo install stonktop
```

### باستعمال cargo-binstall

إذا كان [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) مثبّتًا على جهازك، يمكنك تنصيب الملفات الثنائية الجاهزة مباشرة:

```bash
cargo binstall stonktop
```

### الملفات الثنائية الجاهزة

قم بتحميل الملفات الثنائية الجاهزة من صفحة [الإصدارات (Releases)](https://github.com/somethingwithproof/stonktop/releases).

المنصات المتوفرة:
- Linux (`x86_64`، `aarch64`، `musl`)
- macOS (`Intel`، `Apple Silicon`)
- Windows (`x86_64`، `aarch64`)

### التجميع من الشيفرة المصدرية

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## الاستعمال

### الاستعمال الأساسي

```bash
# تتبع رموز مالية محددة
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# باستعمال اختصارات العملات المشفرة
stonktop -s AAPL,BTC,ETH,SOL
```

### خيارات شبيهة بأداة `top`

```bash
# ضبط مهلة التحديث بالثواني (مثل top -d)
stonktop -s AAPL,GOOGL -d 10

# تنفيذ N دورة تحديث ثم الخروج (مثل top -n)
stonktop -s AAPL -n 5

# الوضع الدفعي للسكربتات (مثل top -b)
stonktop -s AAPL,GOOGL -b

# الوضع الآمن - تعطيل الأوامر التفاعلية
stonktop -s AAPL -S
```

### خيارات الفرز والترتيب

```bash
# الفرز حسب الثمن (تنازلي)
stonktop -s AAPL,GOOGL -o price

# الفرز حسب الرمز (تصاعدي)
stonktop -s AAPL,GOOGL -o symbol -r

# حقول الفرز المتوفرة: symbol, name, price, change, change-percent, volume, market-cap
```

### ملف التهيئة

```bash
# استعمال ملف تهيئة مخصص
stonktop -c ~/.config/stonktop/custom.toml

# أو وضع ملف التهيئة في المسار الافتراضي:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## خيارات سطر الأوامر

| الخيار | الاختصار | الوصف |
|--------|----------|-------|
| `--symbols` | `-s` | لائحة الرموز المراد تتبعها مفصولة بفواصل |
| `--delay` | `-d` | مهلة التحديث بالثواني (افتراضيًا: 5) |
| `--iterations` | `-n` | عدد دورات التحديث (`0` = غير محدود) |
| `--batch` | `-b` | الوضع الدفعي - مخرجات غير تفاعلية |
| `--secure` | `-S` | الوضع الآمن - تعطيل الأوامر التفاعلية |
| `--config` | `-c` | مسار ملف التهيئة |
| `--sort` | `-o` | حقل الفرز الابتدائي |
| `--reverse` | `-r` | عكس ترتيب الفرز |
| `--holdings` | `-H` | عرض واجهة المحفظة والأصول المملوكة |
| `--currency` | | عملة العرض (افتراضيًا: `USD`) |
| `--timeout` | | المهلة القصوى لاستجابة واجهة API بالثواني (افتراضيًا: 10) |
| `--format` | | صيغة الإخراج في الوضع الدفعي: `table` أو `json` أو `csv` |
| `--color` | | تلوين المخرجات: `auto` أو `always` أو `never` |
| `--verbose` | `-v` | إخراج مفصّل |
| `--init` | | توليد ملف تهيئة نموذجي |
| `--force` | | فرض استبدال ملف التهيئة القائم (رفقة `--init`) |
| `--lang` | | لغة العرض (`STONKTOP_LANG`، افتراضيًا: تلقائي من `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | إظهار رسالة المساعدة |
| `--version` | `-V` | عرض رقم الإصدار |

## اللغات المدعومة

يرصد Stonktop لغة العرض تلقائيًا انطلاقًا من `STONKTOP_LANG` و`LC_ALL` و`LC_MESSAGES` و`LANG`، كما يقبل الخيار `--lang <LANG>`. تضم جميع اللغات الـ 45 المدعومة ترجمة كاملة لواجهة سطر الأوامر والواجهة الطرفية (بما في ذلك المحاذاة من اليمين إلى اليسار للعربية والفارسية والعبرية والأردية) إضافة إلى توثيق مخصص لكل لغة:

| الرمز | اللغة | التوثيق | الرمز | اللغة | التوثيق | الرمز | اللغة | التوثيق |
|-------|-------|---------|-------|-------|---------|-------|-------|---------|
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

## اختصارات لوحة المفاتيح

| المفتاح | العملية |
|---------|---------|
| `q`, `Esc` | الخروج |
| `h`, `?` | إظهار أو إخفاء المساعدة |
| `↑`, `k` | الصعود إلى الأعلى |
| `↓`, `j` | النزول إلى الأسفل |
| `g`, `Home` | الانتقال إلى أعلى اللائحة |
| `G`, `End` | الانتقال إلى أسفل اللائحة |
| `PgUp` | صفحة إلى الأعلى |
| `PgDn` | صفحة إلى الأسفل |
| `s` | التبديل بين حقول الفرز |
| `r` | عكس ترتيب الفرز |
| `1-7` | الفرز حسب العمود |
| `Enter`, `d` | عرض نافذة تفاصيل الرمز المحدد |
| `/` | البحث وتصفية الرموز |
| `a` | إضافة رمز إلى لائحة التتبع |
| `x` | حذف الرمز المحدد |
| `S` | إظهار أو إخفاء الشريط الجانبي للمبيانات المصغّرة |
| `e` | تصدير لائحة التتبع بصيغة JSON |
| `H` | التبديل إلى عرض المحفظة |
| `f` | إظهار أو إخفاء المعطيات الأساسية |
| `Space`, `R` | تحديث فوري للمعطيات |
| `Tab` | التنقل بين مجموعات الرموز |

## التهيئة

أنشئ ملف تهيئة بصيغة TOML:

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

# أصول المحفظة المالية (اختياري)
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

# مجموعات الرموز
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# العرض بعملة متعددة مع التحويل (اختياري)
[currency]
display = "EUR"
convert = true

# تنبيهات الأثمنة (اختياري)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# اختصارات مخصصة للعملات المشفرة (اختياري)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## صيغ الرموز

| الصيغة | الوصف | مثال |
|--------|-------|------|
| الأسهم | رمز التداول القياسي | `AAPL`, `GOOGL` |
| العملات المشفرة | مرفقة بلاحقة `USD` | `BTC-USD`, `ETH-USD` |
| الاختصار الموجز للعملات المشفرة | يتوسّع تلقائيًا إلى `-USD` | `BTC.X` -> `BTC-USD` |
| الاختصار التلقائي للعملات المشفرة | العملات المشفرة الرائجة | `BTC` -> `BTC-USD` |

## مصدر المعطيات

يستعمل Stonktop واجهة برمجة التطبيقات Yahoo Finance API لجلب التسعيرات الآنية، دون الحاجة إلى أي مفتاح API.

## التطوير

### التجميع (Build)

```bash
# تجميع نسخة التصحيح (Debug)
cargo build

# تجميع نسخة الإصدار (Release)
cargo build --release
```

### الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل الاختبارات مع عرض المخرجات
cargo test -- --nocapture
```

### التدقيق البرمجي (Lint)

```bash
# التحقق من أخطاء وتحذيرات التدقيق
cargo clippy --all-targets --all-features

# تطبيق التصحيحات التلقائية
cargo clippy --fix --allow-dirty
```

### تنسيق الشيفرة

```bash
# فحص مطابقة التنسيق
cargo fmt -- --check

# تطبيق التنسيق التلقائي
cargo fmt
```

## المساهمة

المساهمات مرحب بها! لا تتردد في تقديم طلب دمج (Pull Request).

1. انسخ المستودع إلى حسابك (Fork)
2. أنشئ فرعًا خاصًا بالميزة (`git checkout -b feature/amazing-feature`)
3. اعتمد تغييراتك (`git commit -m 'Add amazing feature'`)
4. ادفع الفرع إلى المستودع (`git push origin feature/amazing-feature`)
5. افتح طلب دمج (Pull Request)

## الرخصة

يخضع هذا المشروع لرخصة MIT - راجع ملف [LICENSE](../../../LICENSE) لمزيد من التفاصيل.

## شكر وتقدير

- مستلهم من [ticker](https://github.com/achannarasappa/ticker)
- مطوّر باستعمال [ratatui](https://github.com/ratatui-org/ratatui)
- المعطيات مقدّمة من [Yahoo Finance](https://finance.yahoo.com/)
