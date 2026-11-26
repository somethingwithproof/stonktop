# Stonktop

> [English](../../../README.md)

Ujongano lwetheminali olufana ne-`top` lokubeka iliso kumaxabiso ezabelo (stocks) nakwiimali zedijithali (cryptocurrency) ngexesha langoku.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Iimpawu

- **Ukubeka Iliso Kumaxabiso Ngexesha Langoku**: Amaxabiso angoku ezabelo nakwiimali zedijithali
- **Ujongano Olufana ne-Top**: Ulawulo lwekhibhodi oluqhelekileyo olufana nolwe-`top`/`htop`
- **Iitshathi ze-Sparkline**: Umboniso wokuhamba kwexabiso kumgca omncinci kwibha esecaleni
- **Ukulandelela Ipotifoliyo**: Beka iliso kwizabelo onazo kunye nezibalo zenzuzo nelahleko (P/L)
- **Ukukhangela kunye Nokucoca**: Ukukhangela okungacaluliyo oonobumba abakhulu nabancinci kwiimpawu nakumagama
- **Izilumkiso Zamaxabiso**: Izaziso zedesktop xa amaxabiso egqitha kwimida emisiweyo
- **Ukuthumela Ngaphandle/Ukungenisa**: Gcina uze ulayishe uluhlu lokulandelela njenge-JSON okanye i-CSV
- **Iimali Ezininzi**: Bonisa amaxabiso ngayo nayiphi na imali ngamaxabiso okutshintshiselana angoku
- **Iindlela Ezininzi Zokuhlela**: Hlela ngophawu, ixabiso, utshintsho, umthamo (volume), okanye ixabiso lemakethi (market cap)
- **Imowudi Yeqela (Batch Mode)**: Iziphumo ezingasebenzisaniyo zezikripthi (njenge-`top -b`)
- **Uqwalaselo lwe-TOML**: Iifayile zoqwalaselo ezilula ukuzihlela
- **Imibala Ebonisayo**: Oluhlaza wenzuzo, obomvu welahleko
- **Iindlela Ezimfutshane Ze-Crypto**: Sebenzisa i-`BTC.X` okanye i-`BTC` nje endaweni ye-`BTC-USD`

## Imifanekiso Yeskrini

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

## Izinto Ezifunekayo Ngaphambili

- **Rust**: Inguqulelo 1.89 okanye ngaphezulu (MSRV)
- **Cargo**: Ibandakanyiwe kufakelo lwe-Rust

## Ufakelo

### Ukusuka kwi-crates.io (Kuyacetyiswa)

```bash
cargo install stonktop
```

### Ukusebenzisa i-cargo-binstall

Ukuba unayo i-[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) efakiweyo, unokufaka iibhayinari esele zakhiwe:

```bash
cargo binstall stonktop
```

### Iibhayinari Esele Zakhiwe

Khuphela iibhayinari esele zakhiwe kwiphepha le-[Releases](https://github.com/somethingwithproof/stonktop/releases).

Amaqonga akhoyo:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Ukusuka Kwikhowudi Yomthombo

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Ukusetyenziswa

### Ukusetyenziswa Okusisiseko

```bash
# Beka iliso kwiimpawu ezithile
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Ngeendlela ezimfutshane ze-crypto
stonktop -s AAPL,BTC,ETH,SOL
```

### Iinketho Ezifana ne-Top

```bash
# Misela ixesha lokuhlaziya (njenge-top -d)
stonktop -s AAPL,GOOGL -d 10

# Sebenzisa imijikelo engu-N uze uphume (njenge-top -n)
stonktop -s AAPL -n 5

# Imowudi yeqela yezikripthi (njenge-top -b)
stonktop -s AAPL,GOOGL -b

# Imowudi ekhuselekileyo - vala imiyalelo esebenzisanayo
stonktop -s AAPL -S
```

### Iinketho Zokuhlela

```bash
# Hlela ngexabiso (ukuhla)
stonktop -s AAPL,GOOGL -o price

# Hlela ngophawu (ukunyuka)
stonktop -s AAPL,GOOGL -o symbol -r

# Imimandla yokuhlela ekhoyo: symbol, name, price, change, change-percent, volume, market-cap
```

### Ifayile Yoqwalaselo

```bash
# Sebenzisa ifayile ethile yoqwalaselo
stonktop -c ~/.config/stonktop/custom.toml

# Okanye beka ifayile yoqwalaselo kwindawo emiselweyo:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Iinketho Zomgca Womyalelo

| Inketho | Emfutshane | Inkcazelo |
|---------|------------|-----------|
| `--symbols` | `-s` | Uluhlu lweempawu ezahlulwe ngekoma zokubeka iliso |
| `--delay` | `-d` | Ixesha lokuhlaziya ngemizuzwana (okumiselweyo: 5) |
| `--iterations` | `-n` | Inani lemijikelo (0 = ngaphandle kwesiphelo) |
| `--batch` | `-b` | Imowudi yeqela - iziphumo ezingasebenzisaniyo |
| `--secure` | `-S` | Imowudi ekhuselekileyo - vala imiyalelo esebenzisanayo |
| `--config` | `-c` | Umendo wefayile yoqwalaselo |
| `--sort` | `-o` | Ummandla wokuqala wokuhlela |
| `--reverse` | `-r` | Guqula ulandelelwano lokuhlela |
| `--holdings` | `-H` | Bonisa umboniso wezabelo/ipotifoliyo |
| `--currency` | | Imali eboniswayo (okumiselweyo: USD) |
| `--timeout` | | Ixesha lokuphelelwa kwe-API ngemizuzwana (okumiselweyo: 10) |
| `--format` | | Ifomathi yeziphumo zemowudi yeqela: table, json, csv |
| `--color` | | Iziphumo zombala: auto, always, never |
| `--verbose` | `-v` | Iziphumo ezineenkcukacha |
| `--init` | | Yenza ifayile engumzekelo yoqwalaselo |
| `--force` | | Nyanzelisa ukubhala phezu koqwalaselo olukhoyo (kunye ne-`--init`) |
| `--lang` | | Ulwimi lokubonisa (`STONKTOP_LANG`, okumiselweyo: ngokuzenzekelayo kwi-`LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Bonisa umyalezo woncedo |
| `--version` | `-V` | Bonisa inguqulelo |

## Iilwimi Ezixhaswayo

I-Stonktop ibona ulwimi lokubonisa kwi-`STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, kunye ne-`LANG`, okanye yamkela i-`--lang <LANG>`. Zonke iilokheyli ezixhaswayo ezingama-48 ziquka iinguqulelo ezipheleleyo ze-CLI ne-TUI (kubandakanya ulungelelwaniso olusuka ekunene ukuya ekhohlo lwesiArabhu, isiPersi, isiHebhere, nesiUrdu) kunye namaxwebhu aguqulelweyo:

| Ikhowudi | Ulwimi | README | Ikhowudi | Ulwimi | README | Ikhowudi | Ulwimi | README |
|----------|--------|--------|----------|--------|--------|----------|--------|--------|
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

## Iindlela Ezimfutshane Zekhibhodi

| Iqhosha | Isenzo |
|---------|--------|
| `q`, `Esc` | Phuma |
| `h`, `?` | Vula/vala uncedo |
| `↑`, `k` | Yiya phezulu |
| `↓`, `j` | Yiya ezantsi |
| `g`, `Home` | Yiya encotsheni |
| `G`, `End` | Yiya ezantsi ekugqibeleni |
| `PgUp` | Iphepha elingasentla |
| `PgDn` | Iphepha elingasezantsi |
| `s` | Tshintsha ummandla wokuhlela |
| `r` | Guqula ulandelelwano lokuhlela |
| `1-7` | Hlela ngekholam |
| `Enter` | Bonisa ifestile yeenkcukacha yophawu olukhethiweyo |
| `/` | Khangela/coca iimpawu |
| `a` | Yongeza uphawu kuluhlu lokulandelela |
| `d` | Susa uphawu olukhethiweyo |
| `S` | Vula/vala ibha esecaleni yeetshathi ze-sparkline |
| `e` | Thumela ngaphandle uluhlu lokulandelela kwi-JSON |
| `H` | Vula/vala umboniso wezabelo |
| `f` | Vula/vala iinkcukacha ezisisiseko (fundamentals) |
| `Space`, `R` | Nyanzelisa ukuhlaziya |
| `Tab` | Tshintsha phakathi kwamaqela eempawu |

## Uqwalaselo

Yenza ifayile yoqwalaselo ye-TOML:

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

# Izabelo zepotifoliyo (ngokuzithandela)
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

# Amaqela eempawu
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Umboniso weemali ezininzi (ngokuzithandela)
[currency]
display = "EUR"
convert = true

# Izilumkiso zamaxabiso (ngokuzithandela)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Iindlela ezimfutshane ze-crypto eziqhelekileyo (ngokuzithandela)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Iifomathi Zeempawu

| Ifomathi | Inkcazelo | Umzekelo |
|----------|-----------|----------|
| Izabelo | Uphawu oluqhelekileyo | `AAPL`, `GOOGL` |
| I-Crypto | Inesimamva se-USD | `BTC-USD`, `ETH-USD` |
| Isifinyezo se-Crypto | Sanda sibe ngu--USD | `BTC.X` -> `BTC-USD` |
| I-Crypto ezenzekelayo | Iimali ze-crypto eziqhelekileyo | `BTC` -> `BTC-USD` |

## Umthombo Wedatha

I-Stonktop isebenzisa i-Yahoo Finance API ukufumana amaxabiso exesha langoku. Akukho sitshixo se-API sifunekayo.

## Uphuhliso

### Ukwakha (Build)

```bash
# Ukwakha kokuhlola iimpazamo (Debug)
cargo build

# Ukwakha kokukhutshwa (Release)
cargo build --release
```

### Uvavanyo (Test)

```bash
# Sebenzisa zonke iimvavanyo
cargo test

# Sebenzisa iimvavanyo ngeziphumo ezibonakalayo
cargo test -- --nocapture
```

### Ukuhlola Ikhowudi (Lint)

```bash
# Hlola iingxaki zekhowudi
cargo clippy --all-targets --all-features

# Sebenzisa ulungiso oluzenzekelayo
cargo clippy --fix --allow-dirty
```

### Ukufomatha (Format)

```bash
# Hlola ukufomathwa
cargo fmt -- --check

# Sebenzisa ukufomatha
cargo fmt
```

## Ukufaka Isandla

Ukufaka isandla kwamkelekile! Nceda ukhululeke ukuthumela i-Pull Request.

1. Yenza ifolokhwe (Fork) yokugcina ikhowudi
2. Yenza isebe lakho lophawu (`git checkout -b feature/amazing-feature`)
3. Gcina utshintsho lwakho (`git commit -m 'Add amazing feature'`)
4. Thumela isebe (`git push origin feature/amazing-feature`)
5. Vula i-Pull Request

## Ilayisensi

Le projekthi inelayisensi phantsi kwe-MIT License - jonga ifayile ye-[LICENSE](../../../LICENSE) ngeenkcukacha.

## Imibulelo

- Ikhuthazwe yi-[ticker](https://github.com/achannarasappa/ticker)
- Yakhiwe nge-[ratatui](https://github.com/ratatui-org/ratatui)
- Idatha ivela kwi-[Yahoo Finance](https://finance.yahoo.com/)
