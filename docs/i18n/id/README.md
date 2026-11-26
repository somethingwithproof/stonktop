# Stonktop

> [English](../../../README.md)

Antarmuka terminal (TUI) bergaya `top` untuk memantau harga saham dan mata uang kripto secara waktu nyata.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Fitur

- **Pemantauan Harga Waktu Nyata**: Kuotasi langsung untuk saham dan mata uang kripto
- **Antarmuka Bergaya `top`**: Kontrol papan ketik yang familier serupa `top`/`htop`
- **Grafik Sparkline**: Visualisasi tren harga sebaris pada bilah sisi
- **Pelacakan Portofolio**: Pantau kepemilikan aset Anda beserta perhitungan laba/rugi (P/L)
- **Pencarian & Penyaringan**: Pencarian yang tidak peka huruf besar/kecil di seluruh simbol dan nama
- **Peringatan Harga**: Notifikasi desktop saat harga melewati ambang batas yang dikonfigurasi
- **Ekspor/Impor**: Simpan dan muat daftar pantauan dalam format JSON atau CSV
- **Multi-Mata Uang**: Tampilkan harga dalam mata uang apa pun dengan kurs konversi langsung
- **Beragam Opsi Pengurutan**: Urutkan berdasarkan simbol, harga, perubahan, volume, atau kapitalisasi pasar
- **Mode Batch**: Keluaran non-interaktif untuk skrip otomasi (seperti `top -b`)
- **Konfigurasi TOML**: Berkas konfigurasi yang mudah disunting
- **Kode Warna**: Hijau untuk kenaikan, merah untuk penurunan
- **Pintasan Kripto**: Gunakan `BTC.X` atau cukup `BTC` untuk `BTC-USD`

## Tangkapan Layar

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

- **Rust**: Versi 1.89 atau lebih tinggi (MSRV)
- **Cargo**: Sudah termasuk dalam instalasi Rust

## Instalasi

### Dari crates.io (Disarankan)

```bash
cargo install stonktop
```

### Menggunakan cargo-binstall

Jika Anda telah memasang [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), Anda dapat memasang biner prakompilasi secara langsung:

```bash
cargo binstall stonktop
```

### Biner Prakompilasi

Unduh biner prakompilasi dari halaman [Rilis](https://github.com/somethingwithproof/stonktop/releases).

Platform yang tersedia:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Dari Kode Sumber

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Penggunaan

### Penggunaan Dasar

```bash
# Pantau simbol tertentu
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Menggunakan pintasan kripto
stonktop -s AAPL,BTC,ETH,SOL
```

### Opsi Bergaya top

```bash
# Atur jeda pembaruan (seperti top -d)
stonktop -s AAPL,GOOGL -d 10

# Jalankan sebanyak N iterasi lalu keluar (seperti top -n)
stonktop -s AAPL -n 5

# Mode batch untuk skrip otomasi (seperti top -b)
stonktop -s AAPL,GOOGL -b

# Mode aman - nonaktifkan perintah interaktif
stonktop -s AAPL -S
```

### Opsi Pengurutan

```bash
# Urutkan berdasarkan harga (menurun)
stonktop -s AAPL,GOOGL -o price

# Urutkan berdasarkan simbol (menaik)
stonktop -s AAPL,GOOGL -o symbol -r

# Kolom pengurutan yang tersedia: symbol, name, price, change, change-percent, volume, market-cap
```

### Berkas Konfigurasi

```bash
# Gunakan berkas konfigurasi tertentu
stonktop -c ~/.config/stonktop/custom.toml

# Atau letakkan berkas konfigurasi di lokasi bawaan:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opsi Baris Perintah

| Opsi | Singkat | Deskripsi |
|------|---------|-----------|
| `--symbols` | `-s` | Daftar simbol yang dipantau, dipisahkan koma |
| `--delay` | `-d` | Jeda pembaruan dalam detik (bawaan: 5) |
| `--iterations` | `-n` | Jumlah iterasi (0 = tak terbatas) |
| `--batch` | `-b` | Mode batch - keluaran non-interaktif |
| `--secure` | `-S` | Mode aman - nonaktifkan perintah interaktif |
| `--config` | `-c` | Jalur menuju berkas konfigurasi |
| `--sort` | `-o` | Kolom pengurutan awal |
| `--reverse` | `-r` | Balikkan urutan pengurutan |
| `--holdings` | `-H` | Tampilkan tampilan kepemilikan/portofolio |
| `--currency` | | Mata uang tampilan (bawaan: USD) |
| `--timeout` | | Batas waktu API dalam detik (bawaan: 10) |
| `--format` | | Format keluaran untuk mode batch: table, json, csv |
| `--color` | | Keluaran warna: auto, always, never |
| `--verbose` | `-v` | Keluaran terperinci |
| `--init` | | Hasilkan contoh berkas konfigurasi |
| `--force` | | Paksa timpa konfigurasi yang ada (bersama `--init`) |
| `--lang` | | Bahasa tampilan (`STONKTOP_LANG`, bawaan: otomatis dari `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Tampilkan pesan bantuan |
| `--version` | `-V` | Tampilkan versi |

## Bahasa yang Didukung

Stonktop mendeteksi bahasa tampilan dari `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, dan `LANG`, atau menerima opsi `--lang <LANG>`. Seluruh 48 lokal yang didukung mencakup terjemahan CLI dan TUI lengkap (termasuk perataan kanan-ke-kiri untuk bahasa Arab, Persia, Ibrani, dan Urdu) serta dokumentasi terlokalisasi:

| Kode | Bahasa | README | Kode | Bahasa | README | Kode | Bahasa | README |
|------|--------|--------|------|--------|--------|------|--------|--------|
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

## Pintasan Papan Ketik

| Tombol | Tindakan |
|--------|----------|
| `q`, `Esc` | Keluar |
| `h`, `?` | Buka/tutup bantuan |
| `↑`, `k` | Pindah ke atas |
| `↓`, `j` | Pindah ke bawah |
| `g`, `Home` | Pergi ke paling atas |
| `G`, `End` | Pergi ke paling bawah |
| `PgUp` | Satu halaman ke atas |
| `PgDn` | Satu halaman ke bawah |
| `s` | Ganti kolom pengurutan |
| `r` | Balikkan urutan pengurutan |
| `1-7` | Urutkan berdasarkan kolom |
| `Enter` | Tampilkan jendela detail untuk simbol terpilih |
| `/` | Cari/saring simbol |
| `a` | Tambahkan simbol ke daftar pantauan |
| `d` | Hapus simbol terpilih |
| `S` | Buka/tutup bilah sisi grafik sparkline |
| `e` | Ekspor daftar pantauan ke JSON |
| `H` | Buka/tutup tampilan kepemilikan |
| `f` | Buka/tutup data fundamental |
| `Space`, `R` | Paksa muat ulang |
| `Tab` | Ganti grup simbol |

## Konfigurasi

Buat berkas konfigurasi TOML:

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

# Kepemilikan portofolio (opsional)
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

# Grup simbol
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Tampilan multi-mata uang (opsional)
[currency]
display = "EUR"
convert = true

# Peringatan harga (opsional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Pintasan kripto khusus (opsional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Format Simbol

| Format | Deskripsi | Contoh |
|--------|-----------|--------|
| Saham | Simbol ticker standar | `AAPL`, `GOOGL` |
| Kripto | Dengan akhiran USD | `BTC-USD`, `ETH-USD` |
| Singkatan kripto | Diperluas otomatis menjadi -USD | `BTC.X` -> `BTC-USD` |
| Kripto otomatis | Mata uang kripto umum | `BTC` -> `BTC-USD` |

## Sumber Data

Stonktop menggunakan Yahoo Finance API untuk mengambil kuotasi secara waktu nyata. Tidak diperlukan kunci API.

## Pengembangan

### Kompilasi

```bash
# Kompilasi mode debug
cargo build

# Kompilasi mode rilis
cargo build --release
```

### Pengujian

```bash
# Jalankan seluruh pengujian
cargo test

# Jalankan pengujian dengan menampilkan keluaran
cargo test -- --nocapture
```

### Pemeriksaan Kode (Lint)

```bash
# Periksa masalah lint pada kode
cargo clippy --all-targets --all-features

# Terapkan perbaikan otomatis
cargo clippy --fix --allow-dirty
```

### Pemformatan Kode

```bash
# Periksa pemformatan
cargo fmt -- --check

# Terapkan pemformatan
cargo fmt
```

## Kontribusi

Kontribusi sangat disambut baik! Silakan ajukan Pull Request.

1. Fork repositori ini
2. Buat cabang fitur Anda (`git checkout -b feature/amazing-feature`)
3. Commit perubahan Anda (`git commit -m 'Add amazing feature'`)
4. Push cabang ke repositori jarak jauh (`git push origin feature/amazing-feature`)
5. Buka sebuah Pull Request

## Lisensi

Proyek ini dilisensikan di bawah Lisensi MIT — lihat berkas [LICENSE](../../../LICENSE) untuk detailnya.

## Ucapan Terima Kasih

- Terinspirasi oleh [ticker](https://github.com/achannarasappa/ticker)
- Dibangun menggunakan [ratatui](https://github.com/ratatui-org/ratatui)
- Data berasal dari [Yahoo Finance](https://finance.yahoo.com/)
