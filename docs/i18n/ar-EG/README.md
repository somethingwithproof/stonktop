# Stonktop

> [English](../../../README.md)

واجهة طرفية (Terminal UI) شبيهة بأداة `top` لمتابعة أسعار الأسهم والعملات الرقمية لحظة بلحظة في الوقت الفعلي.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## المميزات

- **متابعة لحظية للأسعار**: أسعار فورية ومباشرة لأسهم البورصة والعملات الرقمية
- **واجهة على نمط `top`**: تحكّم مألوف بلوحة المفاتيح شبيه بأدوات `top`/`htop`
- **رسوم بيانية مصغّرة (Sparkline)**: عرض فوري لاتجاه حركة السعر في شريط جانبي
- **متابعة المحفظة الاستثمارية**: رصد ممتلكاتك مع حساب فوري للأرباح والخسائر (P/L)
- **البحث والتصفية**: بحث سريع غير حساس لحالة الأحرف في الرموز وأسماء الشركات
- **تنبيهات الأسعار**: إشعارات على سطح المكتب أول ما السعر يتخطّى الحدود اللي محددها
- **التصدير والاستيراد**: حفظ وتحميل قوائم المتابعة بصيغة JSON أو CSV
- **دعم العملات المتعددة**: عرض الأسعار بأي عملة مع تحويل تلقائي بأسعار الصرف اللحظية
- **خيارات فرز متعددة**: ترتيب حسب الرمز أو السعر أو التغيّر أو حجم التداول أو القيمة السوقية
- **وضع الدُفعات (Batch Mode)**: مخرجات غير تفاعلية مناسبة للسكربتات والأتمتة (زي `top -b`)
- **إعدادات TOML**: ملفات تهيئة واضحة وسهلة التعديل
- **تمييز بالألوان**: الأخضر للمكاسب والأحمر للخسائر
- **اختصارات العملات الرقمية**: تقدر تكتب `BTC.X` أو ببساطة `BTC` بدل `BTC-USD`

## لقطات الشاشة

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

## المتطلبات الأساسية

- **Rust**: إصدار 1.89 أو أحدث (الحد الأدنى المدعوم MSRV)
- **Cargo**: مدمج تلقائيًا مع تثبيت Rust

## التثبيت

### من crates.io (الطريقة المفضلة)

```bash
cargo install stonktop
```

### باستخدام cargo-binstall

لو عندك [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) متثبّت بالفعل، تقدر تنزّل النسخ التنفيذية الجاهزة مباشرةً:

```bash
cargo binstall stonktop
```

### الملفات التنفيذية الجاهزة

نزّل الملفات التنفيذية الجاهزة من صفحة [الإصدارات (Releases)](https://github.com/somethingwithproof/stonktop/releases).

المنصّات المتاحة:
- Linux (`x86_64`، `aarch64`، `musl`)
- macOS (`Intel`، `Apple Silicon`)
- Windows (`x86_64`، `aarch64`)

### البناء من الكود المصدري

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## طريقة الاستخدام

### الاستخدام الأساسي

```bash
# متابعة رموز أسهم وعملات محددة
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# استخدام اختصارات العملات الرقمية
stonktop -s AAPL,BTC,ETH,SOL
```

### خيارات شبيهة بأداة `top`

```bash
# تحديد فاصل التحديث بالثواني (زي top -d)
stonktop -s AAPL,GOOGL -d 10

# تشغيل عدد N من الدورات وبعدها الخروج (زي top -n)
stonktop -s AAPL -n 5

# وضع الدُفعات للسكربتات (زي top -b)
stonktop -s AAPL,GOOGL -b

# الوضع الآمن - تعطيل الأوامر التفاعلية
stonktop -s AAPL -S
```

### خيارات الترتيب والفرز

```bash
# الفرز حسب السعر (تنازلي)
stonktop -s AAPL,GOOGL -o price

# الفرز حسب الرمز (تصاعدي)
stonktop -s AAPL,GOOGL -o symbol -r

# حقول الفرز المتاحة: symbol, name, price, change, change-percent, volume, market-cap
```

### ملف الإعدادات

```bash
# استخدام ملف إعدادات مخصّص
stonktop -c ~/.config/stonktop/custom.toml

# أو حطّ ملف الإعدادات في المسار الافتراضي:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## خيارات سطر الأوامر

| الخيار | الاختصار | الوصف |
|--------|----------|-------|
| `--symbols` | `-s` | قائمة الرموز المطلوب متابعتها مفصولة بفواصل |
| `--delay` | `-d` | فترة التحديث بالثواني (الافتراضي: 5) |
| `--iterations` | `-n` | عدد دورات التحديث (`0` = مستمر بدون توقّف) |
| `--batch` | `-b` | وضع الدُفعات - مخرجات غير تفاعلية |
| `--secure` | `-S` | الوضع الآمن - تعطيل الأوامر التفاعلية |
| `--config` | `-c` | مسار ملف الإعدادات |
| `--sort` | `-o` | حقل الفرز الابتدائي |
| `--reverse` | `-r` | عكس اتجاه الفرز |
| `--holdings` | `-H` | عرض شاشة المحفظة والممتلكات |
| `--currency` | | عملة العرض (الافتراضي: `USD`) |
| `--timeout` | | مهلة استجابة الـ API بالثواني (الافتراضي: 10) |
| `--format` | | صيغة المخرجات في وضع الدُفعات: `table` أو `json` أو `csv` |
| `--color` | | إخراج الألوان: `auto` أو `always` أو `never` |
| `--verbose` | `-v` | مخرجات تفصيلية |
| `--init` | | إنشاء ملف إعدادات نموذجي |
| `--force` | | استبدال ملف الإعدادات الموجود بالقوة (مع `--init`) |
| `--lang` | | لغة العرض (`STONKTOP_LANG`، الافتراضي: تلقائي من `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | عرض رسالة المساعدة |
| `--version` | `-V` | عرض رقم الإصدار |

## اللغات المدعومة

يتعرّف Stonktop تلقائيًا على لغة العرض من متغيرات البيئة `STONKTOP_LANG` و`LC_ALL` و`LC_MESSAGES` و`LANG`، أو يمكنك تمريرها مباشرة عبر `--lang <LANG>`. تشمل جميع اللغات الـ 45 المدعومة ترجمة كاملة لواجهة سطر الأوامر والواجهة الطرفية (بما في ذلك المحاذاة من اليمين إلى اليسار للعربية والفارسية والعبرية والأردية) إلى جانب توثيق مترجم لكل لغة:

| الرمز | اللغة | الدليل | الرمز | اللغة | الدليل | الرمز | اللغة | الدليل |
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

## اختصارات لوحة المفاتيح

| المفتاح | الإجراء |
|---------|---------|
| `q`, `Esc` | الخروج |
| `h`, `?` | إظهار أو إخفاء المساعدة |
| `↑`, `k` | التحرّك لأعلى |
| `↓`, `j` | التحرّك لأسفل |
| `g`, `Home` | الانتقال لبداية القائمة |
| `G`, `End` | الانتقال لنهاية القائمة |
| `PgUp` | صفحة لأعلى |
| `PgDn` | صفحة لأسفل |
| `s` | التبديل بين حقول الفرز |
| `r` | عكس ترتيب الفرز |
| `1-7` | الفرز حسب العمود |
| `Enter` | عرض نافذة تفاصيل الرمز المحدد |
| `/` | البحث وتصفية الرموز |
| `a` | إضافة رمز لقائمة المتابعة |
| `d` | حذف الرمز المحدد |
| `S` | إظهار أو إخفاء الشريط الجانبي للرسوم المصغّرة |
| `e` | تصدير قائمة المتابعة إلى JSON |
| `H` | التبديل إلى عرض المحفظة |
| `f` | إظهار أو إخفاء البيانات الأساسية |
| `Space`, `R` | تحديث فوري للأسعار |
| `Tab` | التنقّل بين مجموعات الرموز |

## الإعدادات

أنشئ ملف إعدادات بصيغة TOML:

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

# ممتلكات المحفظة الاستثمارية (اختياري)
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

# العرض بعملة أخرى مع التحويل التلقائي (اختياري)
[currency]
display = "EUR"
convert = true

# تنبيهات الأسعار (اختياري)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# اختصارات مخصّصة للعملات الرقمية (اختياري)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## صيغ الرموز

| الصيغة | الوصف | مثال |
|--------|-------|------|
| أسهم البورصة | رمز التداول القياسي | `AAPL`, `GOOGL` |
| عملات رقمية | مع لاحقة `USD` | `BTC-USD`, `ETH-USD` |
| اختصار عملات رقمية | يتحوّل تلقائيًا إلى `-USD` | `BTC.X` -> `BTC-USD` |
| اختصار تلقائي للعملات الرقمية | العملات الرقمية الشائعة | `BTC` -> `BTC-USD` |

## مصدر البيانات

يعتمد Stonktop على واجهة Yahoo Finance API لجلب الأسعار اللحظية، بدون الحاجة لأي مفتاح API.

## التطوير

### البناء

```bash
# بناء وضع التطوير (Debug)
cargo build

# بناء وضع الإصدار (Release)
cargo build --release
```

### الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل الاختبارات مع إظهار المخرجات كاملة
cargo test -- --nocapture
```

### الفحص الساكن (Lint)

```bash
# فحص مشاكل الكود والتحذيرات
cargo clippy --all-targets --all-features

# تطبيق الإصلاحات التلقائية
cargo clippy --fix --allow-dirty
```

### تنسيق الكود

```bash
# التحقق من تنسيق الكود
cargo fmt -- --check

# تطبيق تنسيق الكود
cargo fmt
```

## المساهمة

نرحّب بمساهماتكم! لا تتردد في فتح طلب سحب (Pull Request).

1. اعمل Fork للمستودع
2. أنشئ فرعًا جديدًا للميزة (`git checkout -b feature/amazing-feature`)
3. احفظ تعديلاتك في Commit (`git commit -m 'Add amazing feature'`)
4. ارفع الفرع إلى المستودع (`git push origin feature/amazing-feature`)
5. افتح Pull Request

## الترخيص

هذا المشروع مرخّص بموجب رخصة MIT - راجع ملف [LICENSE](../../../LICENSE) لمعرفة التفاصيل.

## الشكر والتقدير

- مستوحى من [ticker](https://github.com/achannarasappa/ticker)
- مبني باستخدام [ratatui](https://github.com/ratatui-org/ratatui)
- مصدر البيانات من [Yahoo Finance](https://finance.yahoo.com/)
