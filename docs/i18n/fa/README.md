# Stonktop

> [English](../../../README.md)

رابط کاربری ترمینال مشابه `top` برای پایش لحظه‌ای قیمت سهام و رمزارزها.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## ویژگی‌ها

- **پایش لحظه‌ای قیمت**: دریافت نرخ زنده برای سهام و رمزارزها
- **رابط کاربری مشابه تاپ**: کنترل‌های آشنای صفحه‌کلید مشابه `top`/`htop`
- **نمودارهای خطی کوچک (اسپارک‌لاین)**: نمایش درون‌خطی روند قیمت در نوار کناری
- **پیگیری سبد دارایی (پورتفولیو)**: پایش دارایی‌ها به همراه محاسبه سود و زیان
- **جستجو و فیلتر**: جستجوی غیرحساس به حروف بزرگ و کوچک در میان نمادها و نام‌ها
- **هشدارهای قیمت**: اعلان دسکتاپ هنگام عبور قیمت از آستانه‌های پیکربندی‌شده
- **خروجی و ورودی داده**: ذخیره و بارگذاری فهرست‌های پیگیری در قالب جی‌سان یا سی‌اس‌وی
- **پشتیبانی از چند ارز**: نمایش قیمت‌ها به هر ارز دلخواه با نرخ تبدیل لحظه‌ای
- **گزینه‌های متعدد مرتب‌سازی**: مرتب‌سازی بر اساس نماد، قیمت، تغییر، حجم معاملات و ارزش بازار
- **حالت دسته‌ای**: خروجی غیرتعاملی مناسب برای اسکریپت‌نویسی (مشابه `top -b`)
- **پیکربندی تومل**: پرونده‌های پیکربندی با ویرایش آسان
- **کدگذاری رنگی**: سبز برای رشد قیمت و قرمز برای افت قیمت
- **میانبرهای رمزارز**: استفاده از `BTC.X` یا تنها `BTC` به جای `BTC-USD`

## تصاویر صفحه

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

## پیش‌نیازها

- **راست**: نسخه ۱.۸۹ یا بالاتر (حداقل نسخه پشتیبانی‌شده)
- **کارگو**: همراه با نصب راست ارائه می‌شود

## نصب

### از طریق crates.io (پیشنهادی)

```bash
cargo install stonktop
```

### با استفاده از cargo-binstall

اگر [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) را نصب کرده‌اید، می‌توانید فایل‌های باینری ازپیش‌ساخته‌شده را نصب کنید:

```bash
cargo binstall stonktop
```

### فایل‌های باینری ازپیش‌ساخته‌شده

فایل‌های باینری ازپیش‌ساخته‌شده را از صفحه [Releases](https://github.com/somethingwithproof/stonktop/releases) دریافت کنید.

سکوهای پشتیبانی‌شده:
- لینوکس (x86_64، aarch64، musl)
- مک‌اواس (اینتل، اپل سیلیکون)
- ویندوز (x86_64، aarch64)

### ساخت از کد منبع

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## نحوه استفاده

### استفاده پایه

```bash
# پایش نمادهای مشخص
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# با میانبرهای رمزارز
stonktop -s AAPL,BTC,ETH,SOL
```

### گزینه‌های مشابه تاپ

```bash
# تنظیم بازه زمانی بازخوانی (مشابه top -d)
stonktop -s AAPL,GOOGL -d 10

# اجرای N چرخه و سپس خروج (مشابه top -n)
stonktop -s AAPL -n 5

# حالت دسته‌ای برای اسکریپت‌نویسی (مشابه top -b)
stonktop -s AAPL,GOOGL -b

# حالت امن - غیرفعال‌سازی فرمان‌های تعاملی
stonktop -s AAPL -S
```

### گزینه‌های مرتب‌سازی

```bash
# مرتب‌سازی بر اساس قیمت (نزولی)
stonktop -s AAPL,GOOGL -o price

# مرتب‌سازی بر اساس نماد (صعودی)
stonktop -s AAPL,GOOGL -o symbol -r

# فیلدهای مرتب‌سازی موجود: symbol, name, price, change, change-percent, volume, market-cap
```

### پرونده پیکربندی

```bash
# استفاده از پرونده پیکربندی مشخص
stonktop -c ~/.config/stonktop/custom.toml

# یا قرار دادن پرونده پیکربندی در مسیر پیش‌فرض:
# - لینوکس: ~/.config/stonktop/config.toml
# - مک‌اواس: ~/Library/Application Support/stonktop/config.toml
# - ویندوز: %APPDATA%\stonktop\config.toml
```

## گزینه‌های خط فرمان

| گزینه | کوتاه | توضیحات |
|-------|-------|---------|
| `--symbols` | `-s` | فهرست نمادهای پایش با جداکننده ویرگول |
| `--delay` | `-d` | بازه زمانی بازخوانی بر حسب ثانیه (پیش‌فرض: ۵) |
| `--iterations` | `-n` | تعداد چرخه‌ها (۰ = نامحدود) |
| `--batch` | `-b` | حالت دسته‌ای - خروجی غیرتعاملی |
| `--secure` | `-S` | حالت امن - غیرفعال‌سازی فرمان‌های تعاملی |
| `--config` | `-c` | مسیر پرونده پیکربندی |
| `--sort` | `-o` | فیلد اولیه مرتب‌سازی |
| `--reverse` | `-r` | معکوس کردن ترتیب مرتب‌سازی |
| `--holdings` | `-H` | نمایش نمای دارایی‌ها و سبد سرمایه‌گذاری |
| `--currency` | | ارز نمایشی (پیش‌فرض: USD) |
| `--timeout` | | مهلت زمانی درخواست رابط برنامه‌نویسی بر حسب ثانیه (پیش‌فرض: ۱۰) |
| `--format` | | قالب خروجی برای حالت دسته‌ای: table, json, csv |
| `--color` | | خروجی رنگی: auto, always, never |
| `--verbose` | `-v` | خروجی مشروح |
| `--init` | | ایجاد یک پرونده پیکربندی نمونه |
| `--force` | | بازنویسی اجباری پیکربندی موجود (همراه با `--init`) |
| `--lang` | | زبان نمایش (`STONKTOP_LANG`، پیش‌فرض: تشخیص خودکار از `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | نمایش پیام راهنما |
| `--version` | `-V` | نمایش نسخه برنامه |

## زبان‌های پشتیبانی‌شده

استونک‌تاپ زبان نمایش را از متغیرهای محیطی `STONKTOP_LANG`، `LC_ALL`، `LC_MESSAGES` و `LANG` تشخیص می‌دهد یا گزینه `--lang <LANG>` را می‌پذیرد. هر ۴۸ زبان و منطقه پشتیبانی‌شده دارای ترجمه کامل خط فرمان و رابط کاربری ترمینال (از جمله چینش راست‌به‌چپ برای زبان‌های عربی، فارسی، عبری و اردو) و مستندات بومی‌سازی‌شده هستند:

| کد | زبان | README | کد | زبان | README | کد | زبان | README |
|----|------|--------|----|------|--------|----|------|--------|
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

## میانبرهای صفحه‌کلید

| کلید | عملکرد |
|------|--------|
| `q`, `Esc` | خروج |
| `h`, `?` | نمایش یا پنهان‌کردن راهنما |
| `↑`, `k` | حرکت به بالا |
| `↓`, `j` | حرکت به پایین |
| `g`, `Home` | رفتن به ابتدا |
| `G`, `End` | رفتن به انتها |
| `PgUp` | یک صفحه به بالا |
| `PgDn` | یک صفحه به پایین |
| `s` | تغییر فیلد مرتب‌سازی |
| `r` | معکوس کردن ترتیب مرتب‌سازی |
| `1-7` | مرتب‌سازی بر اساس ستون |
| `Enter` | نمایش پنجره جزئیات برای نماد انتخاب‌شده |
| `/` | جستجو و فیلتر نمادها |
| `a` | افزودن نماد به فهرست پیگیری |
| `d` | حذف نماد انتخاب‌شده |
| `S` | نمایش یا پنهان‌کردن نوار کناری نمودارهای خطی کوچک |
| `e` | خروجی گرفتن از فهرست پیگیری به قالب جی‌سان |
| `H` | نمایش یا پنهان‌کردن نمای دارایی‌ها |
| `f` | نمایش یا پنهان‌کردن اطلاعات بنیادی |
| `Space`, `R` | بازخوانی اجباری |
| `Tab` | جابه‌جایی میان گروه‌های نماد |

## پیکربندی

یک پرونده پیکربندی با قالب تومل ایجاد کنید:

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

# دارایی‌های سبد سرمایه‌گذاری (اختیاری)
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

# گروه‌های نماد
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# نمایش چندارزی (اختیاری)
[currency]
display = "EUR"
convert = true

# هشدارهای قیمت (اختیاری)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# میانبرهای سفارشی رمزارز (اختیاری)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## قالب‌های نماد

| قالب | توضیحات | نمونه |
|------|---------|-------|
| سهام | نماد استاندارد بازار | `AAPL`, `GOOGL` |
| رمزارز | همراه با پسوند دلار آمریکا | `BTC-USD`, `ETH-USD` |
| خلاصه رمزارز | به‌صورت خودکار به `-USD` گسترش می‌یابد | `BTC.X` -> `BTC-USD` |
| رمزارز خودکار | رمزارزهای رایج | `BTC` -> `BTC-USD` |

## منبع داده

استونک‌تاپ برای دریافت قیمت‌های لحظه‌ای از رابط برنامه‌نویسی یاهو فایننس استفاده می‌کند. هیچ کلید رابط برنامه‌نویسی مورد نیاز نیست.

## توسعه

### ساخت

```bash
# ساخت اشکال‌زدایی
cargo build

# ساخت نسخه انتشار
cargo build --release
```

### آزمون

```bash
# اجرای همه آزمون‌ها
cargo test

# اجرای آزمون‌ها همراه با نمایش خروجی
cargo test -- --nocapture
```

### بررسی کیفیت کد

```bash
# بررسی خطاها و هشدارهای کیفیت کد
cargo clippy --all-targets --all-features

# اعمال اصلاحات خودکار
cargo clippy --fix --allow-dirty
```

### قالب‌بندی کد

```bash
# بررسی قالب‌بندی
cargo fmt -- --check

# اعمال قالب‌بندی
cargo fmt
```

## مشارکت

از مشارکت شما استقبال می‌کنیم! لطفاً با خیال راحت درخواست ادغام ارسال کنید.

1. مخزن را انشعاب بگیرید
2. شاخه ویژگی خود را ایجاد کنید (`git checkout -b feature/amazing-feature`)
3. تغییرات خود را ثبت کنید (`git commit -m 'Add amazing feature'`)
4. شاخه را به مخزن راه دور ارسال کنید (`git push origin feature/amazing-feature`)
5. یک درخواست ادغام باز کنید

## پروانه

این پروژه تحت پروانه ام‌آی‌تی منتشر شده است — برای جزئیات بیشتر پرونده [LICENSE](../../../LICENSE) را مشاهده کنید.

## قدردانی

- الهام‌گرفته از [ticker](https://github.com/achannarasappa/ticker)
- ساخته‌شده با [ratatui](https://github.com/ratatui-org/ratatui)
- داده‌ها از [Yahoo Finance](https://finance.yahoo.com/)
