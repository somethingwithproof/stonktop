# Stonktop

> [English](../../../README.md)

Ìbánisọ̀rọ̀ ojú-ìbòjú tẹ́mínà tó dàbí `top` fún ṣíṣe àbójútó iye owó ìpín ilé-iṣẹ́ (stocks) àti owó orí-ayélujára (cryptocurrency) ní àkókò gidi.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Àwọn Ẹ̀yà Pàtàkì

- **Àbójútó Iye Owó ní Àkókò Gidi**: Ìsọfúnni iye owó lọ́wọ́lọ́wọ́ fún ìpín ilé-iṣẹ́ àti owó orí-ayélujára
- **Ìbánisọ̀rọ̀ tó dàbí Top**: Àwọn bọ́tìnì ìdarí bọ́ọ̀dù-ìtẹ̀wé tó mọ́lára gẹ́gẹ́ bí `top`/`htop`
- **Àwọn Àtẹ Sparkline**: Ìfihàn ìṣípo iye owó lórí ìlà ní ẹ̀gbẹ́ ojú-ìbòjú
- **Àbójútó Àkójọpọ̀ Ìdókòwò (Portfolio)**: Ṣe àbójútó àwọn ohun-ìní rẹ pẹ̀lú ìṣirò èrè àti àdánù (P/L)
- **Ìwádìí àti Àṣàyàn**: Ìwádìí láìka lẹ́tà ńlá tàbí kékeré sí ní gbogbo àmì àti orúkọ
- **Ìkìlọ̀ Iye Owó**: Ìfitónilétí lórí kọ̀ǹpútà nígbà tí iye owó bá rékọjá ìwọ̀n tí a ṣètò
- **Ìkójáde/Ìkówọlé**: Fi pamọ́ kí o sì gbé àtòjọ àbójútó wọlé gẹ́gẹ́ bí JSON tàbí CSV
- **Ọ̀pọ̀lọpọ̀ Owó Orílẹ̀-èdè**: Fi iye owó hàn ní èyíkéyìí owó pẹ̀lú òṣùwọ̀n pàṣípààrọ̀ lọ́wọ́lọ́wọ́
- **Ọ̀pọ̀lọpọ̀ Àṣàyàn Ìtòlẹ́sẹẹsẹ**: Tò nípa àmì, iye owó, ìyípadà, ìwọ̀n títà (volume), tàbí ìwọ̀n ọjà (market cap)
- **Ìpò Àkójọpọ̀ (Batch Mode)**: Àbájáde aláìní-ìbánisọ̀rọ̀ fún àwọn ìwé-àṣẹ aládàáṣe (bíi `top -b`)
- **Ìṣètò TOML**: Àwọn fáìlì ìṣètò tó rọrùn láti ṣàtúnṣe
- **Àmì Àwọ̀**: Àwọ̀ ewé fún èrè, àwọ̀ pupa fún àdánù
- **Àwọn Àbùjá Owó Orí-ayélujára**: Lo `BTC.X` tàbí `BTC` nìkan fún `BTC-USD`

## Àwọn Àwòrán Ojú-ìbòjú

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

## Àwọn Ohun Tí A Nílò Tẹ́lẹ̀

- **Rust**: Ẹ̀yà 1.89 tàbí jù bẹ́ẹ̀ lọ (MSRV)
- **Cargo**: Ó wà nínú ìfisọ́rí Rust

## Ìfisọ́rí

### Láti crates.io (A Dámọ̀ràn)

```bash
cargo install stonktop
```

### Nípa Lílò cargo-binstall

Tí o bá ti fi [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) sọ́rí kọ̀ǹpútà rẹ, o lè fi àwọn fáìlì aláṣe tí a ti kọ́ sílẹ̀ sọ́rí:

```bash
cargo binstall stonktop
```

### Àwọn Fáìlì Aláṣe Tí A Ti Kọ́ Sílẹ̀

Ṣe ìgbàsílẹ̀ àwọn fáìlì aláṣe tí a ti kọ́ sílẹ̀ láti ojú-ewé [Releases](https://github.com/somethingwithproof/stonktop/releases).

Àwọn ètò kọ̀ǹpútà tí ó wà:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Láti Orísun Kóòdù (From Source)

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Ìlò

### Ìlò Ìpìlẹ̀

```bash
# Ṣe àbójútó àwọn àmì pàtó
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Pẹ̀lú àwọn àbùjá owó orí-ayélujára
stonktop -s AAPL,BTC,ETH,SOL
```

### Àwọn Àṣàyàn tó dàbí Top

```bash
# Ṣètò àkókò ìdádúró ìsọdọ̀tun (bíi top -d)
stonktop -s AAPL,GOOGL -d 10

# Ṣiṣẹ́ fún ìyípo N kí o sì jáde (bíi top -n)
stonktop -s AAPL -n 5

# Ìpò àkójọpọ̀ fún àwọn ìwé-àṣẹ aládàáṣe (bíi top -b)
stonktop -s AAPL,GOOGL -b

# Ìpò ààbò - pa àwọn àṣẹ ìbánisọ̀rọ̀
stonktop -s AAPL -S
```

### Àwọn Àṣàyàn Ìtòlẹ́sẹẹsẹ

```bash
# Tò nípa iye owó (láti òkè sísàlẹ̀)
stonktop -s AAPL,GOOGL -o price

# Tò nípa àmì (láti ìsàlẹ̀ sókè)
stonktop -s AAPL,GOOGL -o symbol -r

# Àwọn pápá ìtòlẹ́sẹẹsẹ tó wà: symbol, name, price, change, change-percent, volume, market-cap
```

### Fáìlì Ìṣètò

```bash
# Lo fáìlì ìṣètò pàtó kan
stonktop -c ~/.config/stonktop/custom.toml

# Tàbí fi fáìlì ìṣètò sí ibi àkọ́yàn:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Àwọn Àṣàyàn Ìlà Àṣẹ (CLI)

| Àṣàyàn | Àbùjá | Àpèjúwe |
|--------|-------|---------|
| `--symbols` | `-s` | Àtòjọ àwọn àmì láti ṣàbójútó tí a fi kọ́mà pín |
| `--delay` | `-d` | Àkókò ìdádúró ìsọdọ̀tun ní ìṣẹ́jú-àáyá (àkọ́yàn: 5) |
| `--iterations` | `-n` | Iye ìyípo (0 = láìlópin) |
| `--batch` | `-b` | Ìpò àkójọpọ̀ - àbájáde aláìní-ìbánisọ̀rọ̀ |
| `--secure` | `-S` | Ìpò ààbò - pa àwọn àṣẹ ìbánisọ̀rọ̀ |
| `--config` | `-c` | Ọ̀nà sí fáìlì ìṣètò |
| `--sort` | `-o` | Pápá ìtòlẹ́sẹẹsẹ ìbẹ̀rẹ̀ |
| `--reverse` | `-r` | Yí ètò ìtòlẹ́sẹẹsẹ padà |
| `--holdings` | `-H` | Fi ìwòye ohun-ìní/àkójọpọ̀ ìdókòwò hàn |
| `--currency` | | Owó ìfihàn (àkọ́yàn: USD) |
| `--timeout` | | Àkókò ìparí API ní ìṣẹ́jú-àáyá (àkọ́yàn: 10) |
| `--format` | | Ìrísí àbájáde fún ìpò àkójọpọ̀: table, json, csv |
| `--color` | | Àbájáde àwọ̀: auto, always, never |
| `--verbose` | `-v` | Àbájáde kíkún |
| `--init` | | Ṣẹ̀dá fáìlì ìṣètò àpẹẹrẹ |
| `--force` | | Fi agbára kọ sórí ìṣètò tó wà tẹ́lẹ̀ (pẹ̀lú `--init`) |
| `--lang` | | Èdè ìfihàn (`STONKTOP_LANG`, àkọ́yàn: aládàáṣe láti `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Fi ìfiránṣẹ́ ìrànlọ́wọ́ hàn |
| `--version` | `-V` | Fi ẹ̀yà ètò hàn |

## Àwọn Èdè Tí A Ṣàtìlẹ́yìn Fún

Stonktop ń ṣàwárí èdè ìfihàn láti `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, àti `LANG`, tàbí kí ó gba `--lang <LANG>`. Gbogbo àwọn agbègbè èdè márùndínláàádọ́ta (48) tí a ṣàtìlẹ́yìn fún ní ìtumọ̀ kíkún fún CLI àti TUI (pẹ̀lú ìtòlẹ́sẹẹsẹ láti ọ̀tún sí òsì fún èdè Lárúbáwá, Páṣíà, Hébérù, àti Úrúdù) àti àwọn ìwé-àkọsílẹ̀ tí a túmọ̀:

| Kóòdù | Èdè | README | Kóòdù | Èdè | README | Kóòdù | Èdè | README |
|-------|-----|--------|-------|-----|--------|-------|-----|--------|
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

## Àwọn Àbùjá Bọ́ọ̀dù-Ìtẹ̀wé

| Bọ́tìnì | Ìṣe |
|---------|-----|
| `q`, `Esc` | Jáde |
| `h`, `?` | Tan/pa ìrànlọ́wọ́ |
| `↑`, `k` | Lọ sókè |
| `↓`, `j` | Lọ sísàlẹ̀ |
| `g`, `Home` | Lọ sí òkè pátápátá |
| `G`, `End` | Lọ sí ìsàlẹ̀ pátápátá |
| `PgUp` | Ojú-ewé sókè |
| `PgDn` | Ojú-ewé sísàlẹ̀ |
| `s` | Yí pápá ìtòlẹ́sẹẹsẹ padà |
| `r` | Yí ètò ìtòlẹ́sẹẹsẹ padà |
| `1-7` | Tò nípa òpó |
| `Enter` | Fi fèrèsé àlàyé kíkún hàn fún àmì tí a yàn |
| `/` | Wá/ṣẹ́ àwọn àmì |
| `a` | Fi àmì kún àtòjọ àbójútó |
| `d` | Yọ àmì tí a yàn kúrò |
| `S` | Tan/pa ẹ̀gbẹ́ ojú-ìbòjú fún àwọn àtẹ sparkline |
| `e` | Kó àtòjọ àbójútó jáde sí JSON |
| `H` | Tan/pa ìwòye ohun-ìní |
| `f` | Tan/pa àwọn ìsọfúnni ìpìlẹ̀ (fundamentals) |
| `Space`, `R` | Fi agbára sọ dọ̀tun |
| `Tab` | Yí láàárín àwọn ẹgbẹ́ àmì |

## Ìṣètò

Ṣẹ̀dá fáìlì ìṣètò TOML kan:

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

# Àwọn ohun-ìní àkójọpọ̀ ìdókòwò (àṣàyàn)
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

# Àwọn ẹgbẹ́ àmì
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Ìfihàn ọ̀pọ̀lọpọ̀ owó orílẹ̀-èdè (àṣàyàn)
[currency]
display = "EUR"
convert = true

# Àwọn ìkìlọ̀ iye owó (àṣàyàn)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Àwọn àbùjá owó orí-ayélujára àdáni (àṣàyàn)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Àwọn Ìrísí Àmì

| Ìrísí | Àpèjúwe | Àpẹẹrẹ |
|-------|---------|--------|
| Ìpín ilé-iṣẹ́ (Stock) | Àmì ọjà déédéé | `AAPL`, `GOOGL` |
| Owó orí-ayélujára | Pẹ̀lú ìparí USD | `BTC-USD`, `ETH-USD` |
| Àbùjá owó orí-ayélujára | Ń fẹ̀ sí -USD | `BTC.X` -> `BTC-USD` |
| Owó orí-ayélujára aládàáṣe | Àwọn owó orí-ayélujára tó gbajúmọ̀ | `BTC` -> `BTC-USD` |

## Orísun Dátà

Stonktop ń lo Yahoo Finance API láti gba iye owó ní àkókò gidi. Kò nílò kọ́kọ́rọ́ API kankan.

## Ìdàgbàsókè Ètò

### Kíkọ́ (Build)

```bash
# Kíkọ́ fún àtúnṣe àṣìṣe (Debug)
cargo build

# Kíkọ́ fún ìtújáde (Release)
cargo build --release
```

### Ìdánwò (Test)

```bash
# Ṣiṣẹ́ gbogbo àwọn ìdánwò
cargo test

# Ṣiṣẹ́ àwọn ìdánwò pẹ̀lú àbájáde kíkún
cargo test -- --nocapture
```

### Àyẹ̀wò Kóòdù (Lint)

```bash
# Ṣàyẹ̀wò fún àwọn ìṣòro kóòdù
cargo clippy --all-targets --all-features

# Lo àwọn àtúnṣe aládàáṣe
cargo clippy --fix --allow-dirty
```

### Ìṣètò Ìrísí Kóòdù (Format)

```bash
# Ṣàyẹ̀wò ìrísí kóòdù
cargo fmt -- --check

# Lo ìrísí kóòdù
cargo fmt
```

## Ìkópa

A ń kí ìkópa rẹ káàbọ̀! Jọ̀wọ́ ní òmìnira láti fi Pull Request ránṣẹ́.

1. Ṣe àdàkọ (Fork) ibi ìpamọ́ kóòdù
2. Ṣẹ̀dá ẹ̀ka ẹ̀yà rẹ (`git checkout -b feature/amazing-feature`)
3. Fi àwọn àyípadà rẹ pamọ́ (`git commit -m 'Add amazing feature'`)
4. Fi ẹ̀ka rẹ ránṣẹ́ (`git push origin feature/amazing-feature`)
5. Ṣí Pull Request kan

## Ìwé-àṣẹ

Iṣẹ́ àkànṣe yìí wà lábẹ́ ìwé-àṣẹ MIT License - wo fáìlì [LICENSE](../../../LICENSE) fún àlàyé kíkún.

## Ìdúpẹ́

- A gba ìwúrí láti ọ̀dọ̀ [ticker](https://github.com/achannarasappa/ticker)
- A kọ́ ọ pẹ̀lú [ratatui](https://github.com/ratatui-org/ratatui)
- Dátà láti ọ̀dọ̀ [Yahoo Finance](https://finance.yahoo.com/)
