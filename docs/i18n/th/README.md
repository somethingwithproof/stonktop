# Stonktop

> [English](../../../README.md)

ส่วนติดต่อผู้ใช้บนเทอร์มินัลสไตล์ `top` สำหรับติดตามราคาหุ้นและคริปโทเคอร์เรนซีแบบเรียลไทม์

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## คุณสมบัติ

- **การติดตามราคาแบบเรียลไทม์**: ราคาซื้อขายสดสำหรับหุ้นและคริปโทเคอร์เรนซี
- **อินเทอร์เฟซสไตล์ `top`**: การควบคุมผ่านแป้นพิมพ์ที่คุ้นเคยคล้าย `top`/`htop`
- **แผนภูมิสปาร์กไลน์**: แสดงแนวโน้มราคาแบบอินไลน์ในแถบด้านข้าง
- **การติดตามพอร์ตโฟลิโอ**: ติดตามสินทรัพย์ที่ถือครองพร้อมการคำนวณกำไร/ขาดทุน (P/L)
- **การค้นหาและการกรอง**: ค้นหาสัญลักษณ์และชื่อโดยไม่คำนึงถึงตัวพิมพ์ใหญ่-เล็ก
- **การแจ้งเตือนราคา**: แจ้งเตือนบนเดสก์ท็อปเมื่อราคาข้ามเกณฑ์ที่กำหนดไว้
- **การส่งออก/นำเข้า**: บันทึกและโหลดรายการเฝ้าดูในรูปแบบ JSON หรือ CSV
- **รองรับหลายสกุลเงิน**: แสดงราคาในสกุลเงินใดก็ได้พร้อมอัตราแลกเปลี่ยนแบบเรียลไทม์
- **ตัวเลือกการเรียงลำดับหลากหลาย**: เรียงลำดับตามสัญลักษณ์ ราคา การเปลี่ยนแปลง ปริมาณการซื้อขาย และมูลค่าหลักทรัพย์ตามราคาตลาด
- **โหมดแบตช์**: เอาต์พุตแบบไม่โต้ตอบสำหรับการเขียนสคริปต์ (คล้าย `top -b`)
- **การกำหนดค่าด้วย TOML**: ไฟล์กำหนดค่าที่แก้ไขได้ง่าย
- **การแสดงผลด้วยสี**: สีเขียวสำหรับราคาที่ปรับขึ้น สีแดงสำหรับราคาที่ปรับลง
- **ทางลัดคริปโท**: ใช้ `BTC.X` หรือเพียง `BTC` แทน `BTC-USD`

## ภาพหน้าจอ

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

## ข้อกำหนดเบื้องต้น

- **Rust**: เวอร์ชัน 1.89 ขึ้นไป (เวอร์ชันขั้นต่ำที่รองรับ — MSRV)
- **Cargo**: รวมมากับการติดตั้ง Rust

## การติดตั้ง

### จาก crates.io (แนะนำ)

```bash
cargo install stonktop
```

### โดยใช้ cargo-binstall

หากคุณติดตั้ง [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) ไว้แล้ว คุณสามารถติดตั้งไบนารีที่คอมไพล์ไว้ล่วงหน้าได้:

```bash
cargo binstall stonktop
```

### ไบนารีที่คอมไพล์ไว้ล่วงหน้า

ดาวน์โหลดไบนารีที่คอมไพล์ไว้ล่วงหน้าได้จากหน้า [Releases](https://github.com/somethingwithproof/stonktop/releases)

แพลตฟอร์มที่รองรับ:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### จากซอร์สโค้ด

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## การใช้งาน

### การใช้งานพื้นฐาน

```bash
# ติดตามสัญลักษณ์ที่ระบุ
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# ใช้ร่วมกับทางลัดคริปโท
stonktop -s AAPL,BTC,ETH,SOL
```

### ตัวเลือกสไตล์ top

```bash
# กำหนดช่วงเวลารีเฟรช (คล้าย top -d)
stonktop -s AAPL,GOOGL -d 10

# รัน N รอบแล้วออกจากโปรแกรม (คล้าย top -n)
stonktop -s AAPL -n 5

# โหมดแบตช์สำหรับการเขียนสคริปต์ (คล้าย top -b)
stonktop -s AAPL,GOOGL -b

# โหมดปลอดภัย - ปิดใช้งานคำสั่งโต้ตอบ
stonktop -s AAPL -S
```

### ตัวเลือกการเรียงลำดับ

```bash
# เรียงลำดับตามราคา (จากมากไปน้อย)
stonktop -s AAPL,GOOGL -o price

# เรียงลำดับตามสัญลักษณ์ (จากน้อยไปมาก)
stonktop -s AAPL,GOOGL -o symbol -r

# ฟิลด์การเรียงลำดับที่ใช้ได้: symbol, name, price, change, change-percent, volume, market-cap
```

### ไฟล์กำหนดค่า

```bash
# ใช้ไฟล์กำหนดค่าที่ระบุ
stonktop -c ~/.config/stonktop/custom.toml

# หรือวางไฟล์กำหนดค่าไว้ที่ตำแหน่งเริ่มต้น:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## ตัวเลือกบรรทัดคำสั่ง

| ตัวเลือก | แบบย่อ | คำอธิบาย |
|----------|--------|----------|
| `--symbols` | `-s` | รายการสัญลักษณ์ที่ต้องการติดตาม คั่นด้วยเครื่องหมายจุลภาค |
| `--delay` | `-d` | ระยะเวลารีเฟรชเป็นวินาที (ค่าเริ่มต้น: 5) |
| `--iterations` | `-n` | จำนวนรอบการทำงาน (0 = ไม่จำกัด) |
| `--batch` | `-b` | โหมดแบตช์ — เอาต์พุตแบบไม่โต้ตอบ |
| `--secure` | `-S` | โหมดปลอดภัย — ปิดใช้งานคำสั่งโต้ตอบ |
| `--config` | `-c` | พาธไปยังไฟล์กำหนดค่า |
| `--sort` | `-o` | ฟิลด์การเรียงลำดับเริ่มต้น |
| `--reverse` | `-r` | กลับลำดับการเรียง |
| `--holdings` | `-H` | แสดงมุมมองสินทรัพย์ที่ถือครอง/พอร์ตโฟลิโอ |
| `--currency` | | สกุลเงินที่แสดงผล (ค่าเริ่มต้น: USD) |
| `--timeout` | | ระยะเวลาหมดเวลาของ API เป็นวินาที (ค่าเริ่มต้น: 10) |
| `--format` | | รูปแบบเอาต์พุตสำหรับโหมดแบตช์: `table`, `json`, `csv` |
| `--color` | | การแสดงสีของเอาต์พุต: `auto`, `always`, `never` |
| `--verbose` | `-v` | แสดงเอาต์พุตแบบละเอียด |
| `--init` | | สร้างไฟล์กำหนดค่าตัวอย่าง |
| `--force` | | บังคับเขียนทับไฟล์กำหนดค่าที่มีอยู่ (ใช้ร่วมกับ `--init`) |
| `--lang` | | ภาษาที่แสดงผล (`STONKTOP_LANG`, ค่าเริ่มต้น: ตรวจจับอัตโนมัติจาก `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | แสดงข้อความช่วยเหลือ |
| `--version` | `-V` | แสดงเวอร์ชัน |

## ภาษาที่รองรับ

Stonktop ตรวจจับภาษาที่แสดงผลจาก `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` และ `LANG` หรือรับค่าผ่าน `--lang <LANG>` โลแคลที่รองรับทั้งหมด 48 รายการประกอบด้วยคำแปล CLI และ TUI แบบสมบูรณ์ (รวมถึงการจัดแนวจากขวาไปซ้ายสำหรับภาษาอาหรับ เปอร์เซีย ฮีบรู และอูรดู) พร้อมเอกสารประกอบภาษาท้องถิ่น:

| รหัส | ภาษา | README | รหัส | ภาษา | README | รหัส | ภาษา | README |
|------|------|--------|------|------|--------|------|------|--------|
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

## แป้นพิมพ์ลัด

| ปุ่ม | การทำงาน |
|------|----------|
| `q`, `Esc` | ออกจากโปรแกรม |
| `h`, `?` | สลับการแสดงหน้าจอช่วยเหลือ |
| `↑`, `k` | เลื่อนขึ้น |
| `↓`, `j` | เลื่อนลง |
| `g`, `Home` | ไปยังรายการบนสุด |
| `G`, `End` | ไปยังรายการล่างสุด |
| `PgUp` | เลื่อนขึ้นหนึ่งหน้า |
| `PgDn` | เลื่อนลงหนึ่งหน้า |
| `s` | วนเปลี่ยนฟิลด์การเรียงลำดับ |
| `r` | กลับลำดับการเรียง |
| `1-7` | เรียงลำดับตามคอลัมน์ |
| `Enter` | แสดงหน้าต่างรายละเอียดสำหรับสัญลักษณ์ที่เลือก |
| `/` | ค้นหา/กรองสัญลักษณ์ |
| `a` | เพิ่มสัญลักษณ์ลงในรายการเฝ้าดู |
| `d` | ลบสัญลักษณ์ที่เลือก |
| `S` | สลับแถบด้านข้างแผนภูมิสปาร์กไลน์ |
| `e` | ส่งออกรายการเฝ้าดูเป็น JSON |
| `H` | สลับมุมมองสินทรัพย์ที่ถือครอง |
| `f` | สลับการแสดงข้อมูลปัจจัยพื้นฐาน |
| `Space`, `R` | บังคับรีเฟรชข้อมูล |
| `Tab` | วนเปลี่ยนกลุ่มสัญลักษณ์ |

## การกำหนดค่า

สร้างไฟล์กำหนดค่าแบบ TOML:

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

# สินทรัพย์ที่ถือครองในพอร์ตโฟลิโอ (ไม่บังคับ)
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

# กลุ่มสัญลักษณ์
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# การแสดงผลหลายสกุลเงิน (ไม่บังคับ)
[currency]
display = "EUR"
convert = true

# การแจ้งเตือนราคา (ไม่บังคับ)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# ทางลัดคริปโทแบบกำหนดเอง (ไม่บังคับ)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## รูปแบบสัญลักษณ์

| รูปแบบ | คำอธิบาย | ตัวอย่าง |
|--------|----------|----------|
| หุ้น | สัญลักษณ์หุ้นมาตรฐาน | `AAPL`, `GOOGL` |
| คริปโท | มีส่วนต่อท้าย USD | `BTC-USD`, `ETH-USD` |
| ชื่อย่อคริปโท | ขยายเป็น `-USD` | `BTC.X` -> `BTC-USD` |
| คริปโทอัตโนมัติ | คริปโทเคอร์เรนซีทั่วไป | `BTC` -> `BTC-USD` |

## แหล่งข้อมูล

Stonktop ใช้ Yahoo Finance API เพื่อดึงข้อมูลราคาแบบเรียลไทม์ โดยไม่จำเป็นต้องใช้คีย์ API

## การพัฒนา

### การบิลด์

```bash
# บิลด์แบบดีบัก (Debug)
cargo build

# บิลด์แบบเผยแพร่ (Release)
cargo build --release
```

### การทดสอบ

```bash
# รันการทดสอบทั้งหมด
cargo test

# รันการทดสอบพร้อมแสดงเอาต์พุต
cargo test -- --nocapture
```

### การตรวจสอบโค้ด (Lint)

```bash
# ตรวจสอบปัญหาด้านคุณภาพโค้ด
cargo clippy --all-targets --all-features

# ใช้การแก้ไขอัตโนมัติ
cargo clippy --fix --allow-dirty
```

### การจัดรูปแบบโค้ด

```bash
# ตรวจสอบการจัดรูปแบบ
cargo fmt -- --check

# ใช้การจัดรูปแบบ
cargo fmt
```

## การมีส่วนร่วม

ยินดีต้อนรับทุกการมีส่วนร่วม! สามารถส่ง Pull Request เข้ามาได้เลย

1. Fork รีโพซิทอรีนี้
2. สร้างแบรนช์ฟีเจอร์ของคุณ (`git checkout -b feature/amazing-feature`)
3. Commit การเปลี่ยนแปลงของคุณ (`git commit -m 'Add amazing feature'`)
4. Push ไปยังแบรนช์ (`git push origin feature/amazing-feature`)
5. เปิด Pull Request

## สัญญาอนุญาต

โครงการนี้อยู่ภายใต้สัญญาอนุญาต MIT — ดูรายละเอียดเพิ่มเติมได้ในไฟล์ [LICENSE](../../../LICENSE)

## กิตติกรรมประกาศ

- ได้รับแรงบันดาลใจจาก [ticker](https://github.com/achannarasappa/ticker)
- พัฒนาด้วย [ratatui](https://github.com/ratatui-org/ratatui)
- แหล่งข้อมูลจาก [Yahoo Finance](https://finance.yahoo.com/)
