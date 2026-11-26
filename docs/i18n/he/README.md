# Stonktop

> [English](../../../README.md)

ממשק משתמש במסוף בסגנון `top` לניטור מחירי מניות ומטבעות קריפטוגרפיים בזמן אמת.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## תכונות

- **ניטור מחירים בזמן אמת**: שערי שוק חיים עבור מניות ומטבעות קריפטוגרפיים
- **ממשק בסגנון `top`**: מקשי שליטה מוכרים הדומים ל-`top` ול-`htop`
- **גרפי Sparkline**: תצוגה חזותית של מגמות מחיר בסרגל צד
- **מעקב תיק השקעות**: ניטור ההחזקות שלכם כולל חישובי רווח והפסד (P/L)
- **חיפוש וסינון**: חיפוש בלתי תלוי רישיות בין סמלים ושמות
- **התראות מחיר**: התראות שולחן עבודה כאשר מחירים חוצים ערכי סף שהוגדרו
- **ייצוא וייבוא**: שמירה וטעינה של רשימות מעקב בפורמט JSON או CSV
- **תמיכה בריבוי מטבעות**: הצגת מחירים בכל מטבע עם שערי המרה חיים
- **אפשרויות מיון מרובות**: מיון לפי סמל, מחיר, שינוי, נפח מסחר ושווי שוק
- **מצב אצווה**: פלט לא אינטראקטיבי לסקריפטים (בדומה ל-`top -b`)
- **תצורת TOML**: קובצי תצורה נוחים לעריכה
- **קידוד צבעים**: ירוק לעליות, אדום לירידות
- **קיצורי קריפטו**: שימוש ב-`BTC.X` או פשוט `BTC` עבור `BTC-USD`

## צילומי מסך

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

## דרישות קדם

- **Rust**: גרסה 1.89 ומעלה (גרסה נתמכת מזערית — MSRV)
- **Cargo**: כלול בהתקנת Rust

## התקנה

### מתוך crates.io (מומלץ)

```bash
cargo install stonktop
```

### באמצעות cargo-binstall

אם מותקן אצלכם [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), תוכלו להתקין קבצים בינאריים מהודרים מראש:

```bash
cargo binstall stonktop
```

### קבצים בינאריים מהודרים מראש

הורידו קבצים בינאריים מהודרים מראש מדף ה-[Releases](https://github.com/somethingwithproof/stonktop/releases).

פלטפורמות זמינות:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### מהידור מקוד המקור

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## שימוש

### שימוש בסיסי

```bash
# מעקב אחר סמלים מסוימים
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# שימוש בקיצורי קריפטו
stonktop -s AAPL,BTC,ETH,SOL
```

### אפשרויות בסגנון top

```bash
# הגדרת השהיית רענון (בדומה ל-top -d)
stonktop -s AAPL,GOOGL -d 10

# הרצת N איטרציות ולאחר מכן יציאה (בדומה ל-top -n)
stonktop -s AAPL -n 5

# מצב אצווה עבור סקריפטים (בדומה ל-top -b)
stonktop -s AAPL,GOOGL -b

# מצב מאובטח - השבתת פקודות אינטראקטיביות
stonktop -s AAPL -S
```

### אפשרויות מיון

```bash
# מיון לפי מחיר (סדר יורד)
stonktop -s AAPL,GOOGL -o price

# מיון לפי סמל (סדר עולה)
stonktop -s AAPL,GOOGL -o symbol -r

# שדות מיון זמינים: symbol, name, price, change, change-percent, volume, market-cap
```

### קובץ תצורה

```bash
# שימוש בקובץ תצורה מסוים
stonktop -c ~/.config/stonktop/custom.toml

# או מיקום התצורה בנתיב ברירת המחדל:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## אפשרויות שורת הפקודה

| אפשרות | קיצור | תיאור |
|--------|-------|-------|
| `--symbols` | `-s` | רשימת סמלים מופרדת בפסיקים למעקב |
| `--delay` | `-d` | השהיית רענון בשניות (ברירת מחדל: 5) |
| `--iterations` | `-n` | מספר איטרציות (0 = אינסוף) |
| `--batch` | `-b` | מצב אצווה — פלט לא אינטראקטיבי |
| `--secure` | `-S` | מצב מאובטח — השבתת פקודות אינטראקטיביות |
| `--config` | `-c` | נתיב לקובץ התצורה |
| `--sort` | `-o` | שדה המיון ההתחלתי |
| `--reverse` | `-r` | היפוך סדר המיון |
| `--holdings` | `-H` | הצגת תצוגת החזקות ותיק השקעות |
| `--currency` | | מטבע תצוגה (ברירת מחדל: USD) |
| `--timeout` | | פסק זמן של ממשק ה-API בשניות (ברירת מחדל: 10) |
| `--format` | | פורמט פלט במצב אצווה: `table`, `json`, `csv` |
| `--color` | | פלט צבעוני: `auto`, `always`, `never` |
| `--verbose` | `-v` | פלט מפורט |
| `--init` | | יצירת קובץ תצורה לדוגמה |
| `--force` | | כפיית דריסה של תצורה קיימת (בשילוב `--init`) |
| `--lang` | | שפת תצוגה (`STONKTOP_LANG`, ברירת מחדל: זיהוי אוטומטי מ-`LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | הצגת הודעת עזרה |
| `--version` | `-V` | הצגת מספר הגרסה |

## שפות נתמכות

Stonktop מזהה את שפת התצוגה ממשתני הסביבה `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` ו-`LANG`, או מקבל את הדגל `--lang <LANG>`. כל 45 הלוקאלים הנתמכים כוללים תרגום מלא של ממשק שורת הפקודה (CLI) וממשק המסוף (TUI) — לרבות יישור מימין לשמאל עבור ערבית, פרסית, עברית ואורדו — וכן תיעוד מקומי:

| קוד | שפה | README | קוד | שפה | README | קוד | שפה | README |
|-----|-----|--------|-----|-----|--------|-----|-----|--------|
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

## קיצורי מקלדת

| מקש | פעולה |
|-----|-------|
| `q`, `Esc` | יציאה |
| `h`, `?` | הצגה/הסתרה של העזרה |
| `↑`, `k` | מעבר למעלה |
| `↓`, `j` | מעבר למטה |
| `g`, `Home` | מעבר לראש הרשימה |
| `G`, `End` | מעבר לתחתית הרשימה |
| `PgUp` | עמוד למעלה |
| `PgDn` | עמוד למטה |
| `s` | החלפת שדה המיון |
| `r` | היפוך סדר המיון |
| `1-7` | מיון לפי עמודה |
| `Enter` | הצגת חלונית פרטים עבור הסמל הנבחר |
| `/` | חיפוש וסינון סמלים |
| `a` | הוספת סמל לרשימת המעקב |
| `d` | הסרת הסמל הנבחר |
| `S` | הצגה/הסתרה של סרגל הצד לגרפי Sparkline |
| `e` | ייצוא רשימת המעקב ל-JSON |
| `H` | החלפת תצוגת החזקות |
| `f` | הצגה/הסתרה של נתונים פונדמנטליים |
| `Space`, `R` | כפיית רענון |
| `Tab` | מעבר בין קבוצות סמלים |

## תצורה

צרו קובץ תצורה בפורמט TOML:

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

# החזקות תיק השקעות (אופציונלי)
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

# קבוצות סמלים
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# תצוגה בריבוי מטבעות (אופציונלי)
[currency]
display = "EUR"
convert = true

# התראות מחיר (אופציונלי)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# קיצורי קריפטו מותאמים אישית (אופציונלי)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## פורמטים של סמלים

| פורמט | תיאור | דוגמה |
|-------|-------|-------|
| מניה | סמל מסחר תקני | `AAPL`, `GOOGL` |
| קריפטו | עם סיומת USD | `BTC-USD`, `ETH-USD` |
| קיצור קריפטו | מורחב ל-`-USD` | `BTC.X` -> `BTC-USD` |
| קריפטו אוטומטי | מטבעות קריפטו נפוצים | `BTC` -> `BTC-USD` |

## מקור נתונים

Stonktop משתמש בממשק ה-API של Yahoo Finance כדי לאחזר שערי מסחר בזמן אמת. אין צורך במפתח API.

## פיתוח

### בנייה

```bash
# בניית ניפוי שגיאות (Debug)
cargo build

# בניית גרסת הפצה (Release)
cargo build --release
```

### בדיקות

```bash
# הרצת כל הבדיקות
cargo test

# הרצת בדיקות עם הצגת הפלט
cargo test -- --nocapture
```

### ניתוח קוד (Lint)

```bash
# בדיקת בעיות איכות קוד
cargo clippy --all-targets --all-features

# החלת תיקונים אוטומטיים
cargo clippy --fix --allow-dirty
```

### עיצוב קוד

```bash
# בדיקת עיצוב הקוד
cargo fmt -- --check

# החלת עיצוב הקוד
cargo fmt
```

## תרומה לפרויקט

תרומות מתקבלות בברכה! אתם מוזמנים להגיש בקשת משיכה (Pull Request).

1. בצעו Fork למאגר
2. צרו ענף תכונה משלכם (`git checkout -b feature/amazing-feature`)
3. בצעו Commit לשינויים שלכם (`git commit -m 'Add amazing feature'`)
4. דחפו את השינויים לענף (`git push origin feature/amazing-feature`)
5. פתחו בקשת משיכה (Pull Request)

## רישיון

פרויקט זה מורשה תחת רישיון MIT — לפרטים ראו את הקובץ [LICENSE](../../../LICENSE).

## תודות

- בהשראת [ticker](https://github.com/achannarasappa/ticker)
- נבנה באמצעות [ratatui](https://github.com/ratatui-org/ratatui)
- נתונים מאת [Yahoo Finance](https://finance.yahoo.com/)
