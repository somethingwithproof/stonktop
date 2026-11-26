# Stonktop

> [English](../../../README.md)

Терминальный интерфейс в стиле `top` для мониторинга цен акций и криптовалют в реальном времени.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Возможности

- **Мониторинг цен в реальном времени**: актуальные котировки акций и криптовалют
- **Интерфейс в стиле `top`**: привычное управление с клавиатуры, аналогичное `top`/`htop`
- **Спарклайны (мини-графики)**: встроенная визуализация динамики цен на боковой панели
- **Отслеживание портфеля**: контроль активов с расчётом прибыли и убытков (P/L)
- **Поиск и фильтрация**: регистронезависимый поиск по тикерам и названиям
- **Ценовые оповещения**: системные уведомления при пересечении ценой заданных порогов
- **Экспорт и импорт**: сохранение и загрузка списков наблюдения в форматах JSON и CSV
- **Мультивалютность**: отображение цен в любой валюте с конвертацией по текущему курсу
- **Гибкая сортировка**: сортировка по тикеру, цене, изменению, объёму и рыночной капитализации
- **Пакетный режим**: неинтерактивный вывод для сценариев автоматизации (как `top -b`)
- **Конфигурация в формате TOML**: удобные для редактирования файлы конфигурации
- **Цветовая индикация**: зелёный цвет для роста, красный — для снижения
- **Сокращения для криптовалют**: используйте `BTC.X` или просто `BTC` вместо `BTC-USD`

## Снимок экрана

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

## Системные требования

- **Rust**: версия 1.88 или выше (MSRV)
- **Cargo**: входит в стандартную поставку Rust

## Установка

### Из crates.io (рекомендуется)

```bash
cargo install stonktop
```

### С помощью cargo-binstall

Если установлен [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), можно установить готовые бинарные файлы:

```bash
cargo binstall stonktop
```

### Готовые бинарные файлы

Загрузите готовые исполняемые файлы со страницы [Releases](https://github.com/somethingwithproof/stonktop/releases).

Поддерживаемые платформы:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Сборка из исходного кода

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Использование

### Базовое использование

```bash
# Отслеживание указанных тикеров
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# С использованием сокращений для криптовалют
stonktop -s AAPL,BTC,ETH,SOL
```

### Параметры в стиле `top`

```bash
# Задать интервал обновления (аналогично top -d)
stonktop -s AAPL,GOOGL -d 10

# Выполнить N итераций и завершиться (аналогично top -n)
stonktop -s AAPL -n 5

# Пакетный режим для скриптов (аналогично top -b)
stonktop -s AAPL,GOOGL -b

# Безопасный режим — отключение интерактивных команд
stonktop -s AAPL -S
```

### Параметры сортировки

```bash
# Сортировка по цене (по убыванию)
stonktop -s AAPL,GOOGL -o price

# Сортировка по тикеру (по возрастанию)
stonktop -s AAPL,GOOGL -o symbol -r

# Доступные поля сортировки: symbol, name, price, change, change-percent, volume, market-cap
```

### Файл конфигурации

```bash
# Использовать указанный файл конфигурации
stonktop -c ~/.config/stonktop/custom.toml

# Либо разместить файл по пути по умолчанию:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Параметры командной строки

| Параметр | Краткий | Описание |
|----------|---------|----------|
| `--symbols` | `-s` | Список отслеживаемых тикеров через запятую |
| `--delay` | `-d` | Интервал обновления в секундах (по умолчанию: 5) |
| `--iterations` | `-n` | Количество итераций (0 = бесконечно) |
| `--batch` | `-b` | Пакетный режим — неинтерактивный вывод |
| `--secure` | `-S` | Безопасный режим — отключение интерактивных команд |
| `--config` | `-c` | Путь к файлу конфигурации |
| `--sort` | `-o` | Начальное поле сортировки |
| `--reverse` | `-r` | Обратный порядок сортировки |
| `--holdings` | `-H` | Показать представление активов/портфеля |
| `--currency` | | Валюта отображения (по умолчанию: USD) |
| `--timeout` | | Тайм-аут API в секундах (по умолчанию: 10) |
| `--format` | | Формат вывода в пакетном режиме: table, json, csv |
| `--color` | | Цветовой вывод: auto, always, never |
| `--verbose` | `-v` | Подробный вывод |
| `--init` | | Создать образец файла конфигурации |
| `--force` | | Принудительно перезаписать существующую конфигурацию (совместно с `--init`) |
| `--lang` | | Язык интерфейса (`STONKTOP_LANG`, по умолчанию определяется автоматически из `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Показать справку |
| `--version` | `-V` | Показать версию |

## Поддерживаемые языки

Stonktop автоматически определяет язык интерфейса по переменным `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` и `LANG` либо принимает параметр `--lang <LANG>`. Все 45 поддерживаемых локалей включают полный перевод CLI и TUI (в том числе выравнивание справа налево для арабского, персидского, иврита и урду) и локализованную документацию:

| Код | Язык | README | Код | Язык | README | Код | Язык | README |
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

## Горячие клавиши

| Клавиша | Действие |
|---------|----------|
| `q`, `Esc` | Выйти |
| `h`, `?` | Показать/скрыть справку |
| `↑`, `k` | Переместиться вверх |
| `↓`, `j` | Переместиться вниз |
| `g`, `Home` | Перейти в начало списка |
| `G`, `End` | Перейти в конец списка |
| `PgUp` | Страница вверх |
| `PgDn` | Страница вниз |
| `s` | Переключить поле сортировки |
| `r` | Изменить направление сортировки |
| `1-7` | Сортировать по столбцу |
| `Enter`, `d` | Показать подробное окно для выбранного тикера |
| `/` | Поиск/фильтрация тикеров |
| `a` | Добавить тикер в список наблюдения |
| `x` | Удалить выбранный тикер |
| `S` | Показать/скрыть боковую панель спарклайнов |
| `e` | Экспортировать список наблюдения в JSON |
| `H` | Переключить представление портфеля |
| `f` | Показать/скрыть фундаментальные показатели |
| `Space`, `R` | Принудительно обновить данные |
| `Tab` | Переключить группы тикеров |

## Конфигурация

Создайте файл конфигурации в формате TOML:

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

# Активы портфеля (необязательно)
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

# Группы тикеров
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Мультивалютное отображение (необязательно)
[currency]
display = "EUR"
convert = true

# Ценовые оповещения (необязательно)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Пользовательские сокращения для криптовалют (необязательно)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Форматы тикеров

| Формат | Описание | Пример |
|--------|----------|--------|
| Акции | Стандартный тикер | `AAPL`, `GOOGL` |
| Криптовалюты | С суффиксом USD | `BTC-USD`, `ETH-USD` |
| Краткая запись криптовалют | Раскрывается в `-USD` | `BTC.X` -> `BTC-USD` |
| Автоопределение криптовалют | Популярные криптовалюты | `BTC` -> `BTC-USD` |

## Источник данных

Для получения котировок в реальном времени Stonktop использует Yahoo Finance API. Ключ API не требуется.

## Разработка

### Сборка

```bash
# Отладочная сборка
cargo build

# Релизная сборка
cargo build --release
```

### Тестирование

```bash
# Запуск всех тестов
cargo test

# Запуск тестов с выводом в консоль
cargo test -- --nocapture
```

### Статический анализ

```bash
# Проверка линтером
cargo clippy --all-targets --all-features

# Автоматическое исправление замечаний
cargo clippy --fix --allow-dirty
```

### Форматирование

```bash
# Проверка форматирования
cargo fmt -- --check

# Применение форматирования
cargo fmt
```

## Участие в разработке

Мы приветствуем вклад в проект! Будем рады вашим Pull Request.

1. Сделайте форк репозитория
2. Создайте ветку для вашей функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'Add amazing feature'`)
4. Отправьте ветку в удалённый репозиторий (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## Лицензия

Проект распространяется под лицензией MIT — подробности приведены в файле [LICENSE](../../../LICENSE).

## Благодарности

- Вдохновлено проектом [ticker](https://github.com/achannarasappa/ticker)
- Построено на базе [ratatui](https://github.com/ratatui-org/ratatui)
- Данные предоставляются [Yahoo Finance](https://finance.yahoo.com/)
