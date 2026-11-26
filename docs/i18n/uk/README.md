# Stonktop

> [English](../../../README.md)

Термінальний інтерфейс у стилі `top` для моніторингу цін акцій і криптовалют у реальному часі.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Можливості

- **Моніторинг цін у реальному часі**: актуальні котирування акцій і криптовалют
- **Інтерфейс у стилі `top`**: звичне керування з клавіатури, подібне до `top`/`htop`
- **Спарклайни (мініграфіки)**: вбудована візуалізація динаміки цін на бічній панелі
- **Відстеження портфеля**: контроль активів із розрахунком прибутку та збитків (P/L)
- **Пошук і фільтрація**: регістронезалежний пошук за тикерами та назвами
- **Цінові сповіщення**: системні повідомлення при перетині ціною заданих порогів
- **Експорт та імпорт**: збереження й завантаження списків спостереження у форматах JSON і CSV
- **Мультивалютність**: відображення цін у будь-якій валюті з конвертацією за поточним курсом
- **Кілька режимів сортування**: сортування за тикером, ціною, зміною, обсягом і ринковою капіталізацією
- **Пакетний режим**: неінтерактивний вивід для сценаріїв автоматизації (як `top -b`)
- **Конфігурація у форматі TOML**: зручні для редагування файли налаштувань
- **Колірна індикація**: зелений колір для зростання, червоний — для падіння
- **Скорочення для криптовалют**: використовуйте `BTC.X` або просто `BTC` замість `BTC-USD`

## Знімок екрана

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

## Системні вимоги

- **Rust**: версія 1.89 або новіша (MSRV)
- **Cargo**: входить до складу інсталяції Rust

## Встановлення

### Із crates.io (рекомендовано)

```bash
cargo install stonktop
```

### За допомогою cargo-binstall

Якщо встановлено [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), можна встановити готові бінарні файли:

```bash
cargo binstall stonktop
```

### Готові бінарні файли

Завантажте готові виконувані файли зі сторінки [Releases](https://github.com/somethingwithproof/stonktop/releases).

Підтримувані платформи:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Збірка з вихідного коду

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Використання

### Базове використання

```bash
# Відстеження заданих тикерів
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Із використанням скорочень для криптовалют
stonktop -s AAPL,BTC,ETH,SOL
```

### Параметри у стилі `top`

```bash
# Задати інтервал оновлення (подібно до top -d)
stonktop -s AAPL,GOOGL -d 10

# Виконати N ітерацій і завершити роботу (подібно до top -n)
stonktop -s AAPL -n 5

# Пакетний режим для скриптів (подібно до top -b)
stonktop -s AAPL,GOOGL -b

# Безпечний режим — вимкнення інтерактивних команд
stonktop -s AAPL -S
```

### Параметри сортування

```bash
# Сортування за ціною (за спаданням)
stonktop -s AAPL,GOOGL -o price

# Сортування за тикером (за зростанням)
stonktop -s AAPL,GOOGL -o symbol -r

# Доступні поля сортування: symbol, name, price, change, change-percent, volume, market-cap
```

### Файл конфігурації

```bash
# Використати конкретний файл конфігурації
stonktop -c ~/.config/stonktop/custom.toml

# Або розмістити конфігурацію за типовим шляхом:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Параметри командного рядка

| Параметр | Скорочення | Опис |
|----------|------------|------|
| `--symbols` | `-s` | Список тикерів для відстеження через кому |
| `--delay` | `-d` | Інтервал оновлення в секундах (типово: 5) |
| `--iterations` | `-n` | Кількість ітерацій (0 = нескінченно) |
| `--batch` | `-b` | Пакетний режим — неінтерактивний вивід |
| `--secure` | `-S` | Безпечний режим — вимкнення інтерактивних команд |
| `--config` | `-c` | Шлях до файлу конфігурації |
| `--sort` | `-o` | Початкове поле сортування |
| `--reverse` | `-r` | Зворотний порядок сортування |
| `--holdings` | `-H` | Показати подання активів/портфеля |
| `--currency` | | Валюта відображення (типово: USD) |
| `--timeout` | | Тайм-аут API у секундах (типово: 10) |
| `--format` | | Формат виводу для пакетного режиму: table, json, csv |
| `--color` | | Кольоровий вивід: auto, always, never |
| `--verbose` | `-v` | Докладний вивід |
| `--init` | | Згенерувати зразок файлу конфігурації |
| `--force` | | Примусово перезаписати наявну конфігурацію (разом із `--init`) |
| `--lang` | | Мова відображення (`STONKTOP_LANG`, типово визначається автоматично з `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Показати довідку |
| `--version` | `-V` | Показати версію |

## Підтримувані мови

Stonktop визначає мову інтерфейсу за змінними `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` та `LANG` або приймає параметр `--lang <LANG>`. Усі 48 підтримуваних локалей містять повні переклади CLI та TUI (включно з вирівнюванням справа наліво для арабської, перської, івриту та урду) і локалізовану документацію:

| Код | Мова | README | Код | Мова | README | Код | Мова | README |
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

## Комбінації клавіш

| Клавіша | Дія |
|---------|-----|
| `q`, `Esc` | Вийти |
| `h`, `?` | Показати/приховати довідку |
| `↑`, `k` | Переміститися вгору |
| `↓`, `j` | Переміститися вниз |
| `g`, `Home` | Перейти на початок списку |
| `G`, `End` | Перейти в кінець списку |
| `PgUp` | Сторінка вгору |
| `PgDn` | Сторінка вниз |
| `s` | Перемкнути поле сортування |
| `r` | Змінити порядок сортування на зворотний |
| `1-7` | Сортувати за стовпцем |
| `Enter` | Показати вікно деталей для вибраного тикера |
| `/` | Пошук/фільтрація тикерів |
| `a` | Додати тикер до списку спостереження |
| `d` | Видалити вибраний тикер |
| `S` | Показати/приховати бічну панель спарклайнів |
| `e` | Експортувати список спостереження у JSON |
| `H` | Перемкнути подання портфеля |
| `f` | Показати/приховати фундаментальні показники |
| `Space`, `R` | Примусово оновити дані |
| `Tab` | Перемикати групи тикерів |

## Конфігурація

Створіть файл конфігурації у форматі TOML:

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

# Активи портфеля (необов'язково)
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

# Групи тикерів
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Мультивалютне відображення (необов'язково)
[currency]
display = "EUR"
convert = true

# Цінові сповіщення (необов'язково)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Користувацькі скорочення для криптовалют (необов'язково)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Формати тикерів

| Формат | Опис | Приклад |
|--------|------|---------|
| Акції | Стандартний тикер | `AAPL`, `GOOGL` |
| Криптовалюти | Із суфіксом USD | `BTC-USD`, `ETH-USD` |
| Короткий запис криптовалют | Розгортається у `-USD` | `BTC.X` -> `BTC-USD` |
| Автовизначення криптовалют | Популярні криптовалюти | `BTC` -> `BTC-USD` |

## Джерело даних

Для отримання котирувань у реальному часі Stonktop використовує Yahoo Finance API. Ключ API не потрібен.

## Розробка

### Збірка

```bash
# Налагоджувальна збірка
cargo build

# Релізна збірка
cargo build --release
```

### Тестування

```bash
# Запуск усіх тестів
cargo test

# Запуск тестів із виведенням у консоль
cargo test -- --nocapture
```

### Статичний аналіз

```bash
# Перевірка лінтером
cargo clippy --all-targets --all-features

# Автоматичне виправлення зауважень
cargo clippy --fix --allow-dirty
```

### Форматування

```bash
# Перевірка форматування
cargo fmt -- --check

# Застосування форматування
cargo fmt
```

## Участь у розробці

Внески до проєкту вітаються! Надсилайте свої Pull Request.

1. Зробіть форк репозиторію
2. Створіть гілку для нової функції (`git checkout -b feature/amazing-feature`)
3. Зафіксуйте зміни (`git commit -m 'Add amazing feature'`)
4. Надішліть гілку до віддаленого репозиторію (`git push origin feature/amazing-feature`)
5. Відкрийте Pull Request

## Ліцензія

Цей проєкт ліцензовано на умовах ліцензії MIT — докладніше дивіться у файлі [LICENSE](../../../LICENSE).

## Подяки

- Натхненно проєктом [ticker](https://github.com/achannarasappa/ticker)
- Створено за допомогою [ratatui](https://github.com/ratatui-org/ratatui)
- Дані надає [Yahoo Finance](https://finance.yahoo.com/)
