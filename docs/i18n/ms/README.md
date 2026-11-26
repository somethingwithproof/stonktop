# Stonktop

> [English](../../../README.md)

Antara muka pengguna terminal ala `top` untuk memantau harga saham dan mata wang kripto secara masa nyata.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Ciri-ciri

- **Pemantauan Harga Masa Nyata**: Sebut harga langsung untuk saham dan mata wang kripto
- **Antara Muka Ala `top`**: Kawalan papan kekunci yang biasa digunakan seperti `top`/`htop`
- **Carta Sparkline**: Visualisasi arah aliran harga sebaris dalam bar sisi
- **Penjejakan Portfolio**: Pantau pegangan anda berserta pengiraan untung/rugi (P/L)
- **Carian & Penapisan**: Carian tanpa peka huruf besar/kecil merentasi simbol dan nama
- **Makluman Harga**: Pemberitahuan desktop apabila harga melepasi ambang yang dikonfigurasikan
- **Eksport/Import**: Simpan dan muat senarai pantauan dalam format JSON atau CSV
- **Berbilang Mata Wang**: Paparkan harga dalam sebarang mata wang dengan kadar pertukaran langsung
- **Pelbagai Pilihan Isihan**: Isih mengikut simbol, harga, perubahan, jumlah dagangan, permodalan pasaran
- **Mod Kelompok**: Output bukan interaktif untuk penskripan (seperti `top -b`)
- **Konfigurasi TOML**: Fail konfigurasi yang mudah disunting
- **Kod Warna**: Hijau untuk kenaikan, merah untuk penurunan
- **Pintasan Kripto**: Gunakan `BTC.X` atau sekadar `BTC` untuk `BTC-USD`

## Tangkapan Skrin

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

## Prasyarat

- **Rust**: Versi 1.89 atau lebih baharu (MSRV)
- **Cargo**: Disertakan bersama pemasangan Rust

## Pemasangan

### Daripada crates.io (Disyorkan)

```bash
cargo install stonktop
```

### Menggunakan cargo-binstall

Sekiranya anda telah memasang [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), anda boleh memasang binari prabina:

```bash
cargo binstall stonktop
```

### Binari Prabina

Muat turun binari prabina daripada halaman [Releases](https://github.com/somethingwithproof/stonktop/releases).

Platform yang tersedia:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Daripada Kod Sumber

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Penggunaan

### Penggunaan Asas

```bash
# Pantau simbol tertentu
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Dengan pintasan kripto
stonktop -s AAPL,BTC,ETH,SOL
```

### Pilihan Ala top

```bash
# Tetapkan sela segar semula (seperti top -d)
stonktop -s AAPL,GOOGL -d 10

# Jalankan N lelaran kemudian keluar (seperti top -n)
stonktop -s AAPL -n 5

# Mod kelompok untuk penskripan (seperti top -b)
stonktop -s AAPL,GOOGL -b

# Mod selamat - nyahdayakan perintah interaktif
stonktop -s AAPL -S
```

### Pilihan Isihan

```bash
# Isih mengikut harga (menurun)
stonktop -s AAPL,GOOGL -o price

# Isih mengikut simbol (menaik)
stonktop -s AAPL,GOOGL -o symbol -r

# Medan isihan yang tersedia: symbol, name, price, change, change-percent, volume, market-cap
```

### Fail Konfigurasi

```bash
# Gunakan fail konfigurasi tertentu
stonktop -c ~/.config/stonktop/custom.toml

# Atau letakkan fail konfigurasi di lokasi lalai:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Pilihan Baris Perintah

| Pilihan | Ringkas | Penerangan |
|---------|---------|------------|
| `--symbols` | `-s` | Senarai simbol yang dipisahkan dengan koma untuk dipantau |
| `--delay` | `-d` | Sela segar semula dalam saat (lalai: 5) |
| `--iterations` | `-n` | Bilangan lelaran (0 = tanpa had) |
| `--batch` | `-b` | Mod kelompok — output bukan interaktif |
| `--secure` | `-S` | Mod selamat — nyahdayakan perintah interaktif |
| `--config` | `-c` | Laluan ke fail konfigurasi |
| `--sort` | `-o` | Medan isihan awal |
| `--reverse` | `-r` | Songsangkan tertib isihan |
| `--holdings` | `-H` | Paparkan pandangan pegangan/portfolio |
| `--currency` | | Mata wang paparan (lalai: USD) |
| `--timeout` | | Tamat masa API dalam saat (lalai: 10) |
| `--format` | | Format output untuk mod kelompok: `table`, `json`, `csv` |
| `--color` | | Output berwarna: `auto`, `always`, `never` |
| `--verbose` | `-v` | Output terperinci |
| `--init` | | Jana fail konfigurasi contoh |
| `--force` | | Paksa tulis ganti konfigurasi sedia ada (dengan `--init`) |
| `--lang` | | Bahasa paparan (`STONKTOP_LANG`, lalai: automatik daripada `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Paparkan mesej bantuan |
| `--version` | `-V` | Paparkan versi |

## Bahasa yang Disokong

Stonktop mengesan bahasa paparan daripada `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, dan `LANG`, atau menerima `--lang <LANG>`. Kesemua 45 lokal yang disokong merangkumi terjemahan CLI dan TUI sepenuhnya (termasuk penjajaran kanan-ke-kiri untuk bahasa Arab, Parsi, Ibrani dan Urdu) serta dokumentasi setempat:

| Kod | Bahasa | README | Kod | Bahasa | README | Kod | Bahasa | README |
|-----|--------|--------|-----|--------|--------|-----|--------|--------|
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

## Pintasan Papan Kekunci

| Kekunci | Tindakan |
|---------|----------|
| `q`, `Esc` | Keluar |
| `h`, `?` | Togol bantuan |
| `↑`, `k` | Gerak ke atas |
| `↓`, `j` | Gerak ke bawah |
| `g`, `Home` | Pergi ke bahagian atas |
| `G`, `End` | Pergi ke bahagian bawah |
| `PgUp` | Satu halaman ke atas |
| `PgDn` | Satu halaman ke bawah |
| `s` | Tukar medan isihan secara kitaran |
| `r` | Songsangkan tertib isihan |
| `1-7` | Isih mengikut lajur |
| `Enter` | Paparkan tetingkap butiran bagi simbol yang dipilih |
| `/` | Cari/tapis simbol |
| `a` | Tambah simbol ke senarai pantauan |
| `d` | Buang simbol yang dipilih |
| `S` | Togol bar sisi carta sparkline |
| `e` | Eksport senarai pantauan ke JSON |
| `H` | Togol pandangan pegangan |
| `f` | Togol data asas kewangan (fundamentals) |
| `Space`, `R` | Paksa segar semula |
| `Tab` | Tukar kumpulan simbol secara kitaran |

## Konfigurasi

Cipta fail konfigurasi TOML:

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

# Pegangan portfolio (pilihan)
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

# Kumpulan simbol
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Paparan berbilang mata wang (pilihan)
[currency]
display = "EUR"
convert = true

# Makluman harga (pilihan)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Pintasan kripto tersuai (pilihan)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Format Simbol

| Format | Penerangan | Contoh |
|--------|------------|--------|
| Saham | Simbol saham piawai | `AAPL`, `GOOGL` |
| Kripto | Dengan akhiran USD | `BTC-USD`, `ETH-USD` |
| Ringkasan kripto | Dikembangkan kepada `-USD` | `BTC.X` -> `BTC-USD` |
| Kripto automatik | Mata wang kripto lazim | `BTC` -> `BTC-USD` |

## Sumber Data

Stonktop menggunakan Yahoo Finance API untuk mendapatkan sebut harga masa nyata. Tiada kunci API diperlukan.

## Pembangunan

### Bina

```bash
# Binaan nyahpepijat (Debug)
cargo build

# Binaan keluaran (Release)
cargo build --release
```

### Ujian

```bash
# Jalankan semua ujian
cargo test

# Jalankan ujian dengan output
cargo test -- --nocapture
```

### Semakan Kod (Lint)

```bash
# Semak isu linting
cargo clippy --all-targets --all-features

# Gunakan pembetulan automatik
cargo clippy --fix --allow-dirty
```

### Pemformatan

```bash
# Semak pemformatan
cargo fmt -- --check

# Gunakan pemformatan
cargo fmt
```

## Sumbangan

Sumbangan amat dialu-alukan! Sila kemukakan Pull Request.

1. Fork repositori ini
2. Cipta cabang ciri anda (`git checkout -b feature/amazing-feature`)
3. Komit perubahan anda (`git commit -m 'Add amazing feature'`)
4. Tolak ke cabang (`git push origin feature/amazing-feature`)
5. Buka Pull Request

## Lesen

Projek ini dilesenkan di bawah Lesen MIT — lihat fail [LICENSE](../../../LICENSE) untuk butiran lanjut.

## Penghargaan

- Diinspirasikan oleh [ticker](https://github.com/achannarasappa/ticker)
- Dibina dengan [ratatui](https://github.com/ratatui-org/ratatui)
- Data daripada [Yahoo Finance](https://finance.yahoo.com/)
