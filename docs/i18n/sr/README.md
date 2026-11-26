# Stonktop

> [English](../../../README.md)

Терминалски интерфејс налик алату `top` за праћење цена акција и криптовалута у реалном времену.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Функције

- **Праћење цена у реалном времену**: Котације уживо за акције и криптовалуте
- **Интерфејс у стилу алата `top`**: Познате пречице на тастатури сличне алатима `top`/`htop`
- **Спарклајн графикони**: Приказ тренда цена у бочној траци
- **Праћење портфеља**: Пратите своје позиције уз обрачун добити и губитка (P/L)
- **Претрага и филтрирање**: Претрага симбола и назива без обзира на величину слова
- **Упозорења о ценама**: Обавештења на радној површини када цене пређу подешене прагове
- **Извоз/Увоз**: Сачувајте и учитајте листе праћења у JSON или CSV формату
- **Више валута**: Приказ цена у било којој валути уз актуелне курсеве конверзије
- **Више опција сортирања**: Сортирање по симболу, цени, промени, обиму или тржишној капитализацији
- **Пакетни режим**: Неинтерактивни излаз за скрипте (као `top -b`)
- **TOML подешавања**: Датотеке подешавања једноставне за уређивање
- **Означавање бојама**: Зелена за раст, црвена за пад
- **Крипто пречице**: Користите `BTC.X` или само `BTC` за `BTC-USD`

## Снимци екрана

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

## Предуслови

- **Rust**: верзија 1.89 или новија (MSRV)
- **Cargo**: укључен уз инсталацију Rust-а

## Инсталација

### Путем crates.io (препоручено)

```bash
cargo install stonktop
```

### Помоћу cargo-binstall

Ако имате инсталиран [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), можете инсталирати унапред изграђене бинарне датотеке:

```bash
cargo binstall stonktop
```

### Унапред изграђене бинарне датотеке

Преузмите унапред изграђене бинарне датотеке са странице [Издања](https://github.com/somethingwithproof/stonktop/releases).

Доступне платформе:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Из изворног кода

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Коришћење

### Основно коришћење

```bash
# Watch specific symbols
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# With crypto shortcuts
stonktop -s AAPL,BTC,ETH,SOL
```

### Опције налик алату top

```bash
# Set refresh delay (like top -d)
stonktop -s AAPL,GOOGL -d 10

# Run N iterations then exit (like top -n)
stonktop -s AAPL -n 5

# Batch mode for scripting (like top -b)
stonktop -s AAPL,GOOGL -b

# Secure mode - disable interactive commands
stonktop -s AAPL -S
```

### Опције сортирања

```bash
# Sort by price (descending)
stonktop -s AAPL,GOOGL -o price

# Sort by symbol (ascending)
stonktop -s AAPL,GOOGL -o symbol -r

# Available sort fields: symbol, name, price, change, change-percent, volume, market-cap
```

### Датотека подешавања

```bash
# Use specific config file
stonktop -c ~/.config/stonktop/custom.toml

# Or place config at default location:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Опције командне линије

| Опција | Кратко | Опис |
|--------|-------|-------------|
| `--symbols` | `-s` | Листа симбола за праћење одвојена зарезом |
| `--delay` | `-d` | Интервал освежавања у секундама (подразумевано: 5) |
| `--iterations` | `-n` | Број итерација (0 = бесконачно) |
| `--batch` | `-b` | Пакетни режим - неинтерактивни излаз |
| `--secure` | `-S` | Безбедни режим - онемогућава интерактивне команде |
| `--config` | `-c` | Путања до датотеке подешавања |
| `--sort` | `-o` | Почетно поље сортирања |
| `--reverse` | `-r` | Обрнути редослед сортирања |
| `--holdings` | `-H` | Прикажи преглед портфеља и позиција |
| `--currency` | | Валута приказа (подразумевано: USD) |
| `--timeout` | | Временско ограничење API захтева у секундама (подразумевано: 10) |
| `--format` | | Формат излаза за пакетни режим: table, json, csv |
| `--color` | | Излаз у боји: auto, always, never |
| `--verbose` | `-v` | Детаљан излаз |
| `--init` | | Генериши пример датотеке подешавања |
| `--force` | | Принудно препиши постојећа подешавања (уз `--init`) |
| `--lang` | | Језик приказа (`STONKTOP_LANG`, подразумевано: аутоматски из `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Прикажи поруку помоћи |
| `--version` | `-V` | Прикажи верзију |

## Подржани језици

Stonktop препознаје језик приказа из променљивих `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` и `LANG`, или прихвата опцију `--lang <LANG>`. Свих 48 подржаних локалитета укључује потпуне преводе CLI и TUI интерфејса (укључујући поравнање здесна налево за арапски, персијски, хебрејски и урду) и локализовану документацију:

| Код | Језик | README | Код | Језик | README | Код | Језик | README |
|------|----------|--------|------|----------|--------|------|----------|--------|
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
| `zh-TW` | 繁體中文 | [繁體中文](../zh-TW/README.md) | `xh-ZA` | isiXhosa | [isiXhosa](../xh-ZA/README.md) | `sr` | Српски | **Српски** |
| `ar` | العربية | [العربية](../ar/README.md) | `yo-NG` | Yorùbá | [Yorùbá](../yo-NG/README.md) | | | |
| `hi` | हिन्दी | [हिन्दी](../hi/README.md) | `zu-ZA` | isiZulu | [isiZulu](../zu-ZA/README.md) | | | |
| `pl` | Polski | [Polski](../pl/README.md) | | | | | | |
| `tr` | Türkçe | [Türkçe](../tr/README.md) | | | | | | |
| `sv` | Svenska | [Svenska](../sv/README.md) | | | | | | |
| `vi` | Tiếng Việt | [Tiếng Việt](../vi/README.md) | | | | | | |
| `id` | Bahasa Indonesia | [Bahasa Indonesia](../id/README.md) | | | | | | |

## Пречице на тастатури

| Тастер | Радња |
|-----|--------|
| `q`, `Esc` | Излаз |
| `h`, `?` | Прикажи/сакриј помоћ |
| `↑`, `k` | Помери горе |
| `↓`, `j` | Помери доле |
| `g`, `Home` | Иди на врх |
| `G`, `End` | Иди на дно |
| `PgUp` | Страница горе |
| `PgDn` | Страница доле |
| `s` | Промени поље сортирања |
| `r` | Обрни редослед сортирања |
| `1-7` | Сортирај по колони |
| `Enter` | Прикажи детаље за изабрани симбол |
| `/` | Претражи/филтрирај симболе |
| `a` | Додај симбол на листу праћења |
| `d` | Уклони изабрани симбол |
| `S` | Прикажи/сакриј бочну траку са спарклајн графицима |
| `e` | Извези листу праћења у JSON |
| `H` | Прикажи/сакриј преглед позиција |
| `f` | Прикажи/сакриј основне податке |
| `Space`, `R` | Принудно освежи |
| `Tab` | Промени групу симбола |

## Подешавања

Креирајте TOML датотеку подешавања:

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

# Portfolio holdings (optional)
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

# Symbol groups
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Multi-currency display (optional)
[currency]
display = "EUR"
convert = true

# Price alerts (optional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Custom crypto shortcuts (optional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Формати симбола

| Формат | Опис | Пример |
|--------|-------------|---------|
| Акција | Стандардна берзанска ознака | `AAPL`, `GOOGL` |
| Крипто | Са суфиксом USD | `BTC-USD`, `ETH-USD` |
| Крипто скраћеница | Проширује се у -USD | `BTC.X` -> `BTC-USD` |
| Аутоматски крипто | Честе криптовалуте | `BTC` -> `BTC-USD` |

## Извор података

Stonktop користи Yahoo Finance API за преузимање котација у реалном времену. API кључ није потребан.

## Развој

### Изградња

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Тестирање

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Провера кода (Lint)

```bash
# Check for linting issues
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty
```

### Форматирање

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Доприноси

Доприноси су добродошли! Слободно пошаљите Pull Request.

1. Направите fork репозиторијума
2. Креирајте своју грану за функцију (`git checkout -b feature/amazing-feature`)
3. Сачувајте измене (`git commit -m 'Add amazing feature'`)
4. Пошаљите грану на сервер (`git push origin feature/amazing-feature`)
5. Отворите Pull Request

## Лиценца

Овај пројекат је лиценциран под MIT лиценцом - за детаље погледајте датотеку [LICENSE](../../../LICENSE).

## Захвалнице

- Инспирисано алатом [ticker](https://github.com/achannarasappa/ticker)
- Изграђено помоћу [ratatui](https://github.com/ratatui-org/ratatui)
- Подаци са сервиса [Yahoo Finance](https://finance.yahoo.com/)
