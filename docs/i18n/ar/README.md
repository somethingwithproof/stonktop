# Stonktop

> [English](../../../README.md)

واجهة طرفية شبيهة بأداة `top` لمراقبة أسعار الأسهم والعملات الرقمية في الوقت الفعلي.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## الميزات

- **مراقبة الأسعار في الوقت الفعلي**: أسعار فورية للأسهم والعملات الرقمية
- **واجهة شبيهة بأداة `top`**: اختصارات لوحة مفاتيح مألوفة مماثلة لأداتي `top`/`htop`
- **رسوم بيانية مصغّرة (Sparkline)**: عرض مرئي مضمّن لاتجاه الأسعار في شريط جانبي
- **تتبّع المحفظة الاستثمارية**: مراقبة ممتلكاتك المالية مع احتساب الأرباح والخسائر (P/L)
- **البحث والتصفية**: بحث غير حساس لحالة الأحرف عبر الرموز والأسماء
- **تنبيهات الأسعار**: إشعارات سطح المكتب عند تجاوز الأسعار للعتبات المُعدّة مسبقاً
- **التصدير والاستيراد**: حفظ قوائم المتابعة وتحميلها بصيغتي JSON أو CSV
- **تعدّد العملات**: عرض الأسعار بأي عملة مع أسعار صرف فورية
- **خيارات فرز متعدّدة**: الفرز حسب الرمز أو السعر أو التغيّر أو حجم التداول أو القيمة السوقية
- **وضع الدُفعات (Batch Mode)**: مخرجات غير تفاعلية مناسبة للبرمجة النصية (مثل `top -b`)
- **تهيئة عبر TOML**: ملفات تهيئة سهلة التحرير
- **الترميز اللوني**: اللون الأخضر للمكاسب واللون الأحمر للخسائر
- **اختصارات العملات الرقمية**: استخدم `BTC.X` أو ببساطة `BTC` للإشارة إلى `BTC-USD`

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

- **Rust**: الإصدار 1.89 أو أحدث (الحد الأدنى المدعوم لإصدار Rust - MSRV)
- **Cargo**: مُضمّن مع تثبيت Rust

## التثبيت

### من crates.io (موصى به)

```bash
cargo install stonktop
```

### باستخدام cargo-binstall

إذا كانت لديك أداة [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) مُثبّتة، يمكنك تثبيت الملفات التنفيذية المُجمَّعة مسبقاً:

```bash
cargo binstall stonktop
```

### الملفات التنفيذية المُجمَّعة مسبقاً

نزّل الملفات التنفيذية المُجمَّعة مسبقاً من صفحة [الإصدارات (Releases)](https://github.com/somethingwithproof/stonktop/releases).

المنصّات المدعومة:
- Linux (x86_64، aarch64، musl)
- macOS (Intel، Apple Silicon)
- Windows (x86_64، aarch64)

### البناء من الشيفرة المصدرية

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## الاستخدام

### الاستخدام الأساسي

```bash
# مراقبة رموز محدّدة
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# باستخدام اختصارات العملات الرقمية
stonktop -s AAPL,BTC,ETH,SOL
```

### خيارات شبيهة بأداة top

```bash
# تعيين مهلة التحديث بالثواني (مثل top -d)
stonktop -s AAPL,GOOGL -d 10

# تنفيذ N من التكرارات ثم الإنهاء (مثل top -n)
stonktop -s AAPL -n 5

# وضع الدُفعات للبرمجة النصية (مثل top -b)
stonktop -s AAPL,GOOGL -b

# الوضع الآمن - تعطيل الأوامر التفاعلية
stonktop -s AAPL -S
```

### خيارات الفرز

```bash
# الفرز حسب السعر (تنازلياً)
stonktop -s AAPL,GOOGL -o price

# الفرز حسب الرمز (تصاعدياً)
stonktop -s AAPL,GOOGL -o symbol -r

# حقول الفرز المتاحة: symbol, name, price, change, change-percent, volume, market-cap
```

### ملف التهيئة

```bash
# استخدام ملف تهيئة محدّد
stonktop -c ~/.config/stonktop/custom.toml

# أو ضع ملف التهيئة في المسار الافتراضي:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## خيارات سطر الأوامر

| الخيار | الاختصار | الوصف |
|--------|----------|-------|
| `--symbols` | `-s` | قائمة بالرموز المراد مراقبتها مفصولة بفواصل |
| `--delay` | `-d` | مهلة التحديث بالثواني (الافتراضي: 5) |
| `--iterations` | `-n` | عدد التكرارات (0 = غير محدود) |
| `--batch` | `-b` | وضع الدُفعات - مخرجات غير تفاعلية |
| `--secure` | `-S` | الوضع الآمن - تعطيل الأوامر التفاعلية |
| `--config` | `-c` | مسار ملف التهيئة |
| `--sort` | `-o` | حقل الفرز المبدئي |
| `--reverse` | `-r` | عكس ترتيب الفرز |
| `--holdings` | `-H` | عرض الممتلكات والمحفظة الاستثمارية |
| `--currency` | | عملة العرض (الافتراضي: USD) |
| `--timeout` | | مهلة استجابة واجهة البرمجة بالثواني (الافتراضي: 10) |
| `--format` | | صيغة الإخراج لوضع الدُفعات: table أو json أو csv |
| `--color` | | إخراج الألوان: auto أو always أو never |
| `--verbose` | `-v` | مخرجات تفصيلية |
| `--init` | | إنشاء ملف تهيئة نموذجي |
| `--force` | | فرض الكتابة فوق ملف التهيئة الموجود (مع `--init`) |
| `--lang` | | لغة العرض (`STONKTOP_LANG`، الافتراضي: تلقائي من `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | عرض رسالة المساعدة |
| `--version` | `-V` | عرض رقم الإصدار |

## اللغات المدعومة

يكتشف Stonktop لغة العرض تلقائياً من المتغيرات `STONKTOP_LANG` و`LC_ALL` و`LC_MESSAGES` و`LANG`، أو يقبل الخيار `--lang <LANG>`. تتضمّن جميع اللغات الـ 48 المدعومة ترجمة كاملة لواجهتي سطر الأوامر والطرفية (بما في ذلك محاذاة النص من اليمين إلى اليسار للعربية والفارسية والعبرية والأردية) إضافةً إلى التوثيق المُوطَّن:

| الرمز | اللغة | README | الرمز | اللغة | README | الرمز | اللغة | README |
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

## اختصارات لوحة المفاتيح

| المفتاح | الإجراء |
|---------|---------|
| `q`, `Esc` | إنهاء البرنامج |
| `h`, `?` | تبديل عرض المساعدة |
| `↑`, `k` | التحرك للأعلى |
| `↓`, `j` | التحرك للأسفل |
| `g`, `Home` | الانتقال إلى الأعلى |
| `G`, `End` | الانتقال إلى الأسفل |
| `PgUp` | صفحة للأعلى |
| `PgDn` | صفحة للأسفل |
| `s` | التبديل بين حقول الفرز |
| `r` | عكس ترتيب الفرز |
| `1-7` | الفرز حسب العمود |
| `Enter` | عرض نافذة تفاصيل الرمز المحدّد |
| `/` | البحث في الرموز وتصفيتها |
| `a` | إضافة رمز إلى قائمة المتابعة |
| `d` | إزالة الرمز المحدّد |
| `S` | تبديل الشريط الجانبي للرسوم البيانية المصغّرة |
| `e` | تصدير قائمة المتابعة إلى صيغة JSON |
| `H` | تبديل عرض الممتلكات |
| `f` | تبديل عرض المؤشرات الأساسية |
| `Space`, `R` | فرض التحديث الفوري |
| `Tab` | التبديل بين مجموعات الرموز |

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

# العرض متعدّد العملات (اختياري)
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

| الصيغة | الوصف | المثال |
|--------|-------|--------|
| الأسهم | رمز تداول قياسي | `AAPL`، `GOOGL` |
| العملات الرقمية | مع لاحقة USD | `BTC-USD`، `ETH-USD` |
| اختصار العملات الرقمية | يتوسّع تلقائياً إلى -USD | `BTC.X` -> `BTC-USD` |
| التعرف التلقائي للعملات الرقمية | العملات الرقمية الشائعة | `BTC` -> `BTC-USD` |

## مصدر البيانات

يستخدم Stonktop واجهة برمجة تطبيقات Yahoo Finance لجلب الأسعار في الوقت الفعلي، ولا يتطلّب أي مفتاح API.

## التطوير

### البناء

```bash
# بناء وضع التصحيح (Debug)
cargo build

# بناء وضع الإصدار (Release)
cargo build --release
```

### الاختبار

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل الاختبارات مع إظهار المخرجات
cargo test -- --nocapture
```

### التدقيق الساكن للشيفرة (Lint)

```bash
# فحص مشكلات التدقيق الساكن
cargo clippy --all-targets --all-features

# تطبيق الإصلاحات التلقائية
cargo clippy --fix --allow-dirty
```

### تنسيق الشيفرة

```bash
# التحقق من التنسيق
cargo fmt -- --check

# تطبيق التنسيق
cargo fmt
```

## المساهمة

المساهمات محل ترحيب دائم! لا تتردد في تقديم طلب سحب (Pull Request).

1. انسخ المستودع (Fork) إلى حسابك
2. أنشئ فرعاً لميزتك الجديدة (`git checkout -b feature/amazing-feature`)
3. اعتمد تغييراتك (`git commit -m 'Add amazing feature'`)
4. ادفع الفرع إلى المستودع البعيد (`git push origin feature/amazing-feature`)
5. افتح طلب سحب (Pull Request)

## الترخيص

يخضع هذا المشروع لرخصة MIT؛ راجع ملف [LICENSE](../../../LICENSE) للاطلاع على التفاصيل.

## شكر وتقدير

- مستوحى من مشروع [ticker](https://github.com/achannarasappa/ticker)
- مبني باستخدام [ratatui](https://github.com/ratatui-org/ratatui)
- البيانات مقدّمة من [Yahoo Finance](https://finance.yahoo.com/)
