# Stonktop

> [English](../../../README.md)

Isixhumi esibonakalayo setheminali esifana ne-`top` sokuqapha amanani amasheya (stocks) nezimali zedijithali (cryptocurrency) ngesikhathi sangempela.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Izici

- **Ukuqapha Amanani Ngesikhathi Sangempela**: Amanani bukhoma amasheya nezimali zedijithali
- **Isixhumi Esifana ne-Top**: Izilawuli zekhibhodi ezijwayelekile ezifana neze-`top`/`htop`
- **Amashadi e-Sparkline**: Ukuboniswa kokuhamba kwamanani kumugqa omncane kwibha eseceleni
- **Ukulandelela Iphothifoliyo**: Qapha amasheya akho kanye nezibalo zenzuzo nokulahlekelwa (P/L)
- **Ukusesha Nokuhlunga**: Ukusesha okungahlukanisi osonhlamvukazi nabancane ezimpawini nasemagameni
- **Izixwayiso Zamanani**: Izaziso zedeskithophu uma amanani eqela emikhawulweni emisiwe
- **Ukuthumela Ngaphandle/Ukungenisa**: Londoloza bese ulayisha izinhlu zokuqapha njenge-JSON noma i-CSV
- **Izimali Eziningi**: Bonisa amanani kunoma iyiphi imali ngamanani okushintshisana bukhoma
- **Izinketho Eziningi Zokuhlunga**: Hlunga ngophawu, inani, ushintsho, umthamo (volume), noma inani lemakethe (market cap)
- **Imodi Yeqoqo (Batch Mode)**: Okukhiphayo okungasebenzisani okwamasikripthi (njenge-`top -b`)
- **Ukucushwa kwe-TOML**: Amafayela okucushwa alula ukuwahlela
- **Ukufakwa Kwemibala**: Oluhlaza wenzuzo, obomvu wokulahlekelwa
- **Izinqamuleli Ze-Crypto**: Sebenzisa i-`BTC.X` noma i-`BTC` nje esikhundleni se-`BTC-USD`

## Izithombe Zesikrini

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

## Izidingo Zangaphambili

- **Rust**: Inguqulo 1.88 noma ngaphezulu (MSRV)
- **Cargo**: Ifakiwe ekufakweni kwe-Rust

## Ukufaka

### Kusuka ku-crates.io (Kuyanconywa)

```bash
cargo install stonktop
```

### Ngokusebenzisa i-cargo-binstall

Uma unayo i-[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) efakiwe, ungafaka amafayela abhayinari asewakhiwe:

```bash
cargo binstall stonktop
```

### Amafayela Abhayinari Asewakhiwe

Landa amafayela abhayinari asewakhiwe ekhasini le-[Releases](https://github.com/somethingwithproof/stonktop/releases).

Izinkundla ezitholakalayo:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Kusuka Kukhodi Yomthombo

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Ukusetshenziswa

### Ukusetshenziswa Okuyisisekelo

```bash
# Qapha izimpawu ezithile
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Ngezinqamuleli ze-crypto
stonktop -s AAPL,BTC,ETH,SOL
```

### Izinketho Ezifana ne-Top

```bash
# Setha isikhathi sokulinda sokuvuselela (njenge-top -d)
stonktop -s AAPL,GOOGL -d 10

# Sebenzisa imijikelezo engu-N bese uyaphuma (njenge-top -n)
stonktop -s AAPL -n 5

# Imodi yeqoqo yamasikripthi (njenge-top -b)
stonktop -s AAPL,GOOGL -b

# Imodi ephephile - khubaza imiyalo esebenzisanayo
stonktop -s AAPL -S
```

### Izinketho Zokuhlunga

```bash
# Hlunga ngenani (ukwehla)
stonktop -s AAPL,GOOGL -o price

# Hlunga ngophawu (ukwenyuka)
stonktop -s AAPL,GOOGL -o symbol -r

# Izinkambu zokuhlunga ezitholakalayo: symbol, name, price, change, change-percent, volume, market-cap
```

### Ifayela Lokucushwa

```bash
# Sebenzisa ifayela elithile lokucushwa
stonktop -c ~/.config/stonktop/custom.toml

# Noma beka ifayela lokucushwa endaweni ezenzakalelayo:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Izinketho Zomugqa Womyalo (CLI)

| Inketho | Isifinyezo | Incazelo |
|---------|------------|----------|
| `--symbols` | `-s` | Uhlu lwezimpawu ezihlukaniswe ngokhefana zokuqapha |
| `--delay` | `-d` | Isikhathi sokuvuselela ngemizuzwana (okuzenzakalelayo: 5) |
| `--iterations` | `-n` | Inani lemijikelezo (0 = ngaphandle komkhawulo) |
| `--batch` | `-b` | Imodi yeqoqo - okukhiphayo okungasebenzisani |
| `--secure` | `-S` | Imodi ephephile - khubaza imiyalo esebenzisanayo |
| `--config` | `-c` | Indlela eya kufayela lokucushwa |
| `--sort` | `-o` | Inkambu yokuqala yokuhlunga |
| `--reverse` | `-r` | Guqula uhlelo lokuhlunga |
| `--holdings` | `-H` | Bonisa ukubuka kwamasheya/iphothifoliyo |
| `--currency` | | Imali yokubonisa (okuzenzakalelayo: USD) |
| `--timeout` | | Isikhathi sokuphelelwa kwe-API ngemizuzwana (okuzenzakalelayo: 10) |
| `--format` | | Ifomethi yokukhiphayo yemodi yeqoqo: table, json, csv |
| `--color` | | Okukhiphayo kombala: auto, always, never |
| `--verbose` | `-v` | Okukhiphayo okunemininingwane |
| `--init` | | Khiqiza ifayela lokucushwa eliyisampula |
| `--force` | | Phoqa ukubhala phezu kokucushwa okukhona (nge-`--init`) |
| `--lang` | | Ulimi lokubonisa (`STONKTOP_LANG`, okuzenzakalelayo: ngokuzenzakalelayo ku-`LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Bonisa umlayezo wosizo |
| `--version` | `-V` | Bonisa inguqulo |

## Izilimi Ezisekelwayo

I-Stonktop ithola ulimi lokubonisa ku-`STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, kanye ne-`LANG`, noma yamukela i-`--lang <LANG>`. Zonke izindawo zezilimi ezingu-45 ezisekelwayo zifaka phakathi ukuhumusha okuphelele kwe-CLI ne-TUI (kufaka phakathi ukulungelelaniswa okusuka kwesokudla kuya kwesokunxele kwesi-Arabhu, isiPheresiya, isiHebheru, nesi-Urdu) kanye nemibhalo ehumushiwe:

| Ikhodi | Ulimi | README | Ikhodi | Ulimi | README | Ikhodi | Ulimi | README |
|--------|-------|--------|--------|-------|--------|--------|-------|--------|
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

## Izinqamuleli Zekhibhodi

| Ukhiye | Isenzo |
|--------|--------|
| `q`, `Esc` | Phuma |
| `h`, `?` | Vula/vala usizo |
| `↑`, `k` | Yiya phezulu |
| `↓`, `j` | Yiya phansi |
| `g`, `Home` | Yiya phezulu ekugcineni |
| `G`, `End` | Yiya phansi ekugcineni |
| `PgUp` | Ikhasi phezulu |
| `PgDn` | Ikhasi phansi |
| `s` | Shintsha inkambu yokuhlunga |
| `r` | Guqula uhlelo lokuhlunga |
| `1-7` | Hlunga ngekholomu |
| `Enter`, `d` | Bonisa iwindi lemininingwane yophawu olukhethiwe |
| `/` | Sesha/hlunga izimpawu |
| `a` | Engeza uphawu ohlwini lokuqapha |
| `x` | Susa uphawu olukhethiwe |
| `S` | Vula/vala ibha eseceleni yamashadi e-sparkline |
| `e` | Thumela ngaphandle uhlu lokuqapha kwi-JSON |
| `H` | Vula/vala ukubuka kwamasheya |
| `f` | Vula/vala imininingwane eyisisekelo (fundamentals) |
| `Space`, `R` | Phoqa ukuvuselela |
| `Tab` | Shintsha phakathi kwamaqembu ezimpawu |

## Ukucushwa

Dala ifayela lokucushwa le-TOML:

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

# Amasheya ephothifoliyo (ngokuzithandela)
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

# Amaqembu ezimpawu
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Ukuboniswa kwezimali eziningi (ngokuzithandela)
[currency]
display = "EUR"
convert = true

# Izixwayiso zamanani (ngokuzithandela)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Izinqamuleli ze-crypto ezingokwezifiso (ngokuzithandela)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Amafomethi Ezimpawu

| Ifomethi | Incazelo | Isibonelo |
|----------|----------|-----------|
| Amasheya | Uphawu olujwayelekile | `AAPL`, `GOOGL` |
| I-Crypto | Inesijobelelo se-USD | `BTC-USD`, `ETH-USD` |
| Isifinyezo se-Crypto | Sanda sibe ngu--USD | `BTC.X` -> `BTC-USD` |
| I-Crypto ezenzakalelayo | Izimali ze-crypto ezijwayelekile | `BTC` -> `BTC-USD` |

## Umthombo Wedatha

I-Stonktop isebenzisa i-Yahoo Finance API ukulanda amanani ngesikhathi sangempela. Akukho ukhiye we-API odingekayo.

## Ukuthuthukiswa

### Ukwakha (Build)

```bash
# Ukwakha kokulungisa iziphazamiso (Debug)
cargo build

# Ukwakha kokukhishwa (Release)
cargo build --release
```

### Ukuhlola (Test)

```bash
# Sebenzisa zonke izivivinyo
cargo test

# Sebenzisa izivivinyo nokukhiphayo okubonakalayo
cargo test -- --nocapture
```

### Ukuhlola Ikhodi (Lint)

```bash
# Hlola izinkinga zekhodi
cargo clippy --all-targets --all-features

# Sebenzisa ukulungisa okuzenzakalelayo
cargo clippy --fix --allow-dirty
```

### Ukufometha (Format)

```bash
# Hlola ukufomethwa
cargo fmt -- --check

# Sebenzisa ukufometha
cargo fmt
```

## Ukufaka Isandla

Ukufaka isandla kwamukelekile! Sicela ukhululeke ukuthumela i-Pull Request.

1. Yenza ifolokhwe (Fork) yokugcina ikhodi
2. Dala igatsha lakho lesici (`git checkout -b feature/amazing-feature`)
3. Londoloza izinguquko zakho (`git commit -m 'Add amazing feature'`)
4. Thumela igatsha (`git push origin feature/amazing-feature`)
5. Vula i-Pull Request

## Ilayisensi

Le phrojekthi inelayisensi ngaphansi kwe-MIT License - bheka ifayela le-[LICENSE](../../../LICENSE) ukuze uthole imininingwane.

## Ukubonga

- Igqugquzelwe yi-[ticker](https://github.com/achannarasappa/ticker)
- Yakhiwe nge-[ratatui](https://github.com/ratatui-org/ratatui)
- Idatha ivela ku-[Yahoo Finance](https://finance.yahoo.com/)
