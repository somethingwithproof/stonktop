# Stonktop

> [English](../../../README.md)

Is-dhexgal terminal u eg `top` oo loogu talagalay la socodka qiimaha saamiyada iyo lacagaha dijitaalka ah (cryptocurrency) si waqti-dhab ah.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Astaamaha

- **La Socodka Qiimaha Waqtiga-Dhabta ah**: Qiimeyn toos ah oo loogu talagalay saamiyada iyo lacagaha dijitaalka ah
- **Is-dhexgal u eg Top**: Kontoroolka kiiboodhka ee la yaqaan oo la mid ah `top`/`htop`
- **Shaxannada Sparkline**: Muujinta isbeddelka qiimaha ee xariiqda dhinaca (sidebar)
- **La Socodka Maalgashiga (Portfolio)**: La soco hantidaada adigoo leh xisaabinta faa'iidada iyo khasaaraha (P/L)
- **Raadinta & Shaandheynta**: Raadin aan kala soocin xarfaha waaweyn iyo kuwa yaryar oo lagu sameeyo calaamadaha iyo magacyada
- **Digniinaha Qiimaha**: Ogeysiisyada kombiyuutarka marka qiimuhu dhaafo xadka la dejiyay
- **Dhoofinta/Soo-dejinta**: Kaydi oo soo geli liisaska la socodka qaabka JSON ama CSV
- **Lacago Badan**: Ku muuji qiimaha lacag kasta adigoo isticmaalaya sarrifka tooska ah
- **Xulashooyin Kala-soocid Badan**: U kala saar calaamad, qiimo, isbeddel, mug, iyo qiimaha suuqa (market cap)
- **Qaabka Dufcadda (Batch Mode)**: Soo-saar aan is-dhexgal ahayn oo loogu talagalay qoraallada otomaatiga ah (sida `top -b`)
- **Habaynta TOML**: Faylasha habaynta ee si fudud loo tafatiri karo
- **Midabeynta**: Cagaar loogu talagalay kororka, casaan loogu talagalay hoos u dhaca
- **Gaabinta Lacagaha Dijitaalka ah**: Isticmaal `BTC.X` ama kaliya `BTC` halkii aad ka qori lahayd `BTC-USD`

## Sawirrada Shaashadda

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

## Shuruudaha Hore

- **Rust**: Nooca 1.89 ama ka sarreeya (MSRV)
- **Cargo**: Waxay la socotaa rakibidda Rust

## Rakibidda

### Laga soo dejiyo crates.io (La Taliyay)

```bash
cargo install stonktop
```

### Isticmaalka cargo-binstall

Haddii uu kuu rakiban yahay [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), waxaad rakibi kartaa faylasha horay loo dhisay:

```bash
cargo binstall stonktop
```

### Faylasha Horay loo Dhisay (Pre-built Binaries)

Ka soo deji faylasha horay loo dhisay bogga [Releases](https://github.com/somethingwithproof/stonktop/releases).

Nidaamyada la heli karo:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Ka Dhisidda Koodhka Isha (From Source)

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Isticmaalka

### Isticmaalka Aasaasiga ah

```bash
# La soco calaamado gaar ah
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Iyadoo la adeegsanayo gaabinta lacagaha dijitaalka ah
stonktop -s AAPL,BTC,ETH,SOL
```

### Xulashooyinka u eg Top

```bash
# Deji wakhtiga dib u cusboonaysiinta (sida top -d)
stonktop -s AAPL,GOOGL -d 10

# Orod N wareeg kadibna ka bax (sida top -n)
stonktop -s AAPL -n 5

# Qaabka dufcadda ee qoraallada otomaatiga ah (sida top -b)
stonktop -s AAPL,GOOGL -b

# Qaabka amniga - dami amarada is-dhexgalka
stonktop -s AAPL -S
```

### Xulashooyinka Kala-soocidda

```bash
# U kala saar qiimaha (hoos u deg)
stonktop -s AAPL,GOOGL -o price

# U kala saar calaamadda (kor u kac)
stonktop -s AAPL,GOOGL -o symbol -r

# Goobaha kala-soocidda ee jira: symbol, name, price, change, change-percent, volume, market-cap
```

### Faylka Habaynta

```bash
# Isticmaal fayl habayn gaar ah
stonktop -c ~/.config/stonktop/custom.toml

# Ama dhig faylka habaynta goobta caadiga ah:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Xulashooyinka Khadka Amarka (CLI)

| Xulasho | Gaabin | Sharaxaad |
|---------|--------|-----------|
| `--symbols` | `-s` | Liiska calaamadaha la socodka oo kooma u dhaxayso |
| `--delay` | `-d` | Wakhtiga dib u cusboonaysiinta ee ilbiriqsiyada (caadiga ah: 5) |
| `--iterations` | `-n` | Tirada wareegyada (0 = aan dhammaad lahayn) |
| `--batch` | `-b` | Qaabka dufcadda - soo-saar aan is-dhexgal ahayn |
| `--secure` | `-S` | Qaabka amniga - dami amarada is-dhexgalka |
| `--config` | `-c` | Waddada faylka habaynta |
| `--sort` | `-o` | Goobta kala-soocidda ee bilowga ah |
| `--reverse` | `-r` | U rogo habka kala-soocidda |
| `--holdings` | `-H` | Muuji aragtida hantida/maalgashiga |
| `--currency` | | Lacagta lagu muujinayo (caadiga ah: USD) |
| `--timeout` | | Wakhtiga dhammaadka API ee ilbiriqsiyada (caadiga ah: 10) |
| `--format` | | Qaabka soo-saarka ee dufcadda: table, json, csv |
| `--color` | | Soo-saarka midabka: auto, always, never |
| `--verbose` | `-v` | Soo-saar faahfaahsan |
| `--init` | | Samee fayl habayn oo tusaale ah |
| `--force` | | Ku qasab in la beddelo habaynta jirta (oo la socota `--init`) |
| `--lang` | | Luqadda muujinta (`STONKTOP_LANG`, caadiga ah: otomaatig ka yimaada `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Muuji fariinta caawinta |
| `--version` | `-V` | Muuji nooca barnaamijka |

## Luqadaha la Taageero

Stonktop waxa ay si otomaatig ah uga ogaataa luqadda muujinta doorsoomayaasha `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, iyo `LANG`, ama waxay aqbashaa `--lang <LANG>`. Dhammaan 45-ka deegaan-luqadeed ee la taageero waxaa ku jira tarjumaad buuxda oo CLI iyo TUI ah (oo ay ku jirto toosinta midig-ilaa-bidix ee Carabi, Faarsi, Cibraani, iyo Urdu) iyo dukumentiyo ku qoran luqadahaas:

| Koodh | Luqad | README | Koodh | Luqad | README | Koodh | Luqad | README |
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

## Gaabinta Kiiboodhka

| Fure | Hawl |
|------|------|
| `q`, `Esc` | Ka bax |
| `h`, `?` | Daar/dami caawinta |
| `↑`, `k` | Kor u dhaqaaq |
| `↓`, `j` | Hoos u dhaqaaq |
| `g`, `Home` | Tag xagga sare |
| `G`, `End` | Tag xagga hoose |
| `PgUp` | Bog kor u qaad |
| `PgDn` | Bog hoos u dhig |
| `s` | Beddel goobta kala-soocidda |
| `r` | U rogo habka kala-soocidda |
| `1-7` | U kala saar tiirka |
| `Enter` | Muuji daaqadda faahfaahinta ee calaamadda la doortay |
| `/` | Raadi/shaandhee calaamadaha |
| `a` | Ku dar calaamad liiska la socodka |
| `d` | Ka saar calaamadda la doortay |
| `S` | Daar/dami dhinaca shaxannada sparkline |
| `e` | U dhoofi liiska la socodka qaabka JSON |
| `H` | Daar/dami aragtida hantida |
| `f` | Daar/dami xogta aasaasiga ah (fundamentals) |
| `Space`, `R` | Ku qasab dib u cusboonaysiin |
| `Tab` | Beddel kooxaha calaamadaha |

## Habaynta

Samee fayl habayn ah oo TOML ah:

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

# Hantida maalgashiga (ikhtiyaari)
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

# Kooxaha calaamadaha
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Muujinta lacago badan (ikhtiyaari)
[currency]
display = "EUR"
convert = true

# Digniinaha qiimaha (ikhtiyaari)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Gaabinta gaarka ah ee lacagaha dijitaalka ah (ikhtiyaari)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Qaababka Calaamadaha

| Qaab | Sharaxaad | Tusaale |
|------|-----------|---------|
| Saami (Stock) | Calaamad caadi ah | `AAPL`, `GOOGL` |
| Lacag dijitaal ah | Leh dabo-raaca USD | `BTC-USD`, `ETH-USD` |
| Gaabin dijitaal ah | Wuxuu ku fidaa -USD | `BTC.X` -> `BTC-USD` |
| Dijitaal otomaatig ah | Lacagaha dijitaalka ah ee caanka ah | `BTC` -> `BTC-USD` |

## Isha Xogta

Stonktop waxa ay adeegsataa Yahoo Finance API si ay u soo qaadato qiimeynta waqtiga-dhabta ah. Looma baahna fure API.

## Horumarinta

### Dhisidda

```bash
# Dhisidda cilad-saarka (Debug)
cargo build

# Dhisidda sii-daynta (Release)
cargo build --release
```

### Tijaabinta

```bash
# Orod dhammaan tijaabooyinka
cargo test

# Orod tijaabooyinka iyadoo la muujinayo soo-saarka
cargo test -- --nocapture
```

### Hubinta Koodhka (Lint)

```bash
# Hubi dhibaatooyinka koodhka
cargo clippy --all-targets --all-features

# Codso hagaajin otomaatig ah
cargo clippy --fix --allow-dirty
```

### Qaabeynta Koodhka (Format)

```bash
# Hubi qaabeynta
cargo fmt -- --check

# Codso qaabeynta
cargo fmt
```

## Wax ku Darsashada

Wax ku darsashadu way furan tahay! Fadlan xor u noqo inaad soo gudbiso Pull Request.

1. Samee koobbi (Fork) ka mid ah kaydka
2. Samee laantaada astaanta (`git checkout -b feature/amazing-feature`)
3. Kaydi isbeddelladaada (`git commit -m 'Add amazing feature'`)
4. U dir laanta (`git push origin feature/amazing-feature`)
5. Fur Pull Request

## Shatiga

Mashruucan waxaa lagu siiyay shatiga MIT License - eeg faylka [LICENSE](../../../LICENSE) wixii faahfaahin ah.

## Mahadcelin

- Waxaa dhiirrigeliyay [ticker](https://github.com/achannarasappa/ticker)
- Waxaa lagu dhisay [ratatui](https://github.com/ratatui-org/ratatui)
- Xogta waxaa laga helay [Yahoo Finance](https://finance.yahoo.com/)
