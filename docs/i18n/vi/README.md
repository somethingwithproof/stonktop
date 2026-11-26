# Stonktop

> [English](../../../README.md)

Giao diện dòng lệnh (TUI) theo phong cách `top` để theo dõi giá cổ phiếu và tiền mã hóa theo thời gian thực.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Tính năng

- **Theo dõi giá theo thời gian thực**: Cập nhật báo giá trực tiếp cho cổ phiếu và tiền mã hóa
- **Giao diện kiểu `top`**: Điều khiển bàn phím quen thuộc tương tự `top`/`htop`
- **Biểu đồ Sparkline**: Trực quan hóa xu hướng giá ngay trên thanh bên
- **Theo dõi danh mục đầu tư**: Quản lý tài sản nắm giữ cùng tính toán lãi/lỗ (P/L)
- **Tìm kiếm và lọc**: Tìm kiếm không phân biệt chữ hoa/thường theo mã và tên
- **Cảnh báo giá**: Thông báo trên màn hình khi giá vượt qua ngưỡng đã cấu hình
- **Xuất/Nhập dữ liệu**: Lưu và tải danh sách theo dõi dưới định dạng JSON hoặc CSV
- **Đa tiền tệ**: Hiển thị giá bằng bất kỳ loại tiền tệ nào với tỷ giá quy đổi trực tiếp
- **Nhiều tùy chọn sắp xếp**: Sắp xếp theo mã, giá, mức thay đổi, khối lượng giao dịch hoặc vốn hóa thị trường
- **Chế độ hàng loạt (Batch Mode)**: Xuất dữ liệu không tương tác phục vụ viết kịch bản (tương tự `top -b`)
- **Cấu hình TOML**: Tệp cấu hình trực quan, dễ chỉnh sửa
- **Phân biệt bằng màu sắc**: Màu xanh lá cho mức tăng, màu đỏ cho mức giảm
- **Ký hiệu tắt cho tiền mã hóa**: Dùng `BTC.X` hoặc chỉ `BTC` cho `BTC-USD`

## Ảnh chụp màn hình

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

## Yêu cầu hệ thống

- **Rust**: Phiên bản 1.88 trở lên (MSRV)
- **Cargo**: Đi kèm với bộ cài đặt Rust

## Cài đặt

### Từ crates.io (Khuyến nghị)

```bash
cargo install stonktop
```

### Sử dụng cargo-binstall

Nếu bạn đã cài đặt [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), bạn có thể cài đặt trực tiếp tệp thực thi biên dịch sẵn:

```bash
cargo binstall stonktop
```

### Tệp thực thi biên dịch sẵn

Tải xuống tệp thực thi biên dịch sẵn từ trang [Releases](https://github.com/somethingwithproof/stonktop/releases).

Các nền tảng được hỗ trợ:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Biên dịch từ mã nguồn

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Hướng dẫn sử dụng

### Sử dụng cơ bản

```bash
# Theo dõi các mã cụ thể
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Sử dụng ký hiệu tắt cho tiền mã hóa
stonktop -s AAPL,BTC,ETH,SOL
```

### Các tùy chọn kiểu top

```bash
# Thiết lập khoảng thời gian làm mới (tương tự top -d)
stonktop -s AAPL,GOOGL -d 10

# Chạy N lần lặp rồi thoát (tương tự top -n)
stonktop -s AAPL -n 5

# Chế độ hàng loạt cho kịch bản tự động (tương tự top -b)
stonktop -s AAPL,GOOGL -b

# Chế độ bảo mật - vô hiệu hóa các lệnh tương tác
stonktop -s AAPL -S
```

### Tùy chọn sắp xếp

```bash
# Sắp xếp theo giá (giảm dần)
stonktop -s AAPL,GOOGL -o price

# Sắp xếp theo mã (tăng dần)
stonktop -s AAPL,GOOGL -o symbol -r

# Các trường sắp xếp khả dụng: symbol, name, price, change, change-percent, volume, market-cap
```

### Tệp cấu hình

```bash
# Sử dụng tệp cấu hình cụ thể
stonktop -c ~/.config/stonktop/custom.toml

# Hoặc đặt tệp cấu hình tại vị trí mặc định:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Tùy chọn dòng lệnh

| Tùy chọn | Viết tắt | Mô tả |
|----------|----------|-------|
| `--symbols` | `-s` | Danh sách mã cần theo dõi, phân tách bằng dấu phẩy |
| `--delay` | `-d` | Thời gian chờ làm mới tính bằng giây (mặc định: 5) |
| `--iterations` | `-n` | Số lần lặp (0 = vô hạn) |
| `--batch` | `-b` | Chế độ hàng loạt - xuất dữ liệu không tương tác |
| `--secure` | `-S` | Chế độ bảo mật - vô hiệu hóa các lệnh tương tác |
| `--config` | `-c` | Đường dẫn đến tệp cấu hình |
| `--sort` | `-o` | Trường sắp xếp ban đầu |
| `--reverse` | `-r` | Đảo ngược thứ tự sắp xếp |
| `--holdings` | `-H` | Hiển thị chế độ xem danh mục/tài sản nắm giữ |
| `--currency` | | Loại tiền tệ hiển thị (mặc định: USD) |
| `--timeout` | | Thời gian chờ API tính bằng giây (mặc định: 10) |
| `--format` | | Định dạng đầu ra cho chế độ hàng loạt: table, json, csv |
| `--color` | | Chế độ màu đầu ra: auto, always, never |
| `--verbose` | `-v` | Xuất thông tin chi tiết |
| `--init` | | Tạo tệp cấu hình mẫu |
| `--force` | | Ghi đè bắt buộc tệp cấu hình hiện có (dùng cùng `--init`) |
| `--lang` | | Ngôn ngữ hiển thị (`STONKTOP_LANG`, mặc định: tự động từ `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Hiển thị thông báo trợ giúp |
| `--version` | `-V` | Hiển thị phiên bản |

## Ngôn ngữ được hỗ trợ

Stonktop tự động phát hiện ngôn ngữ hiển thị từ `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` và `LANG`, hoặc nhận qua tham số `--lang <LANG>`. Tất cả 45 ngôn ngữ/khu vực được hỗ trợ đều bao gồm bản dịch CLI và TUI đầy đủ (kể cả căn lề từ phải sang trái cho tiếng Ả Rập, tiếng Ba Tư, tiếng Do Thái và tiếng Urdu) cùng tài liệu bản địa hóa:

| Mã | Ngôn ngữ | README | Mã | Ngôn ngữ | README | Mã | Ngôn ngữ | README |
|----|----------|--------|----|----------|--------|----|----------|--------|
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

## Phím tắt

| Phím | Hành động |
|------|-----------|
| `q`, `Esc` | Thoát |
| `h`, `?` | Bật/tắt trợ giúp |
| `↑`, `k` | Di chuyển lên |
| `↓`, `j` | Di chuyển xuống |
| `g`, `Home` | Đi tới đầu danh sách |
| `G`, `End` | Đi tới cuối danh sách |
| `PgUp` | Lên một trang |
| `PgDn` | Xuống một trang |
| `s` | Chuyển đổi trường sắp xếp |
| `r` | Đảo ngược thứ tự sắp xếp |
| `1-7` | Sắp xếp theo cột |
| `Enter`, `d` | Mở hộp thoại chi tiết cho mã đang chọn |
| `/` | Tìm kiếm/lọc mã |
| `a` | Thêm mã vào danh sách theo dõi |
| `x` | Xóa mã đang chọn |
| `S` | Bật/tắt thanh bên biểu đồ sparkline |
| `e` | Xuất danh sách theo dõi ra định dạng JSON |
| `H` | Bật/tắt chế độ xem tài sản nắm giữ |
| `f` | Bật/tắt chỉ số cơ bản |
| `Space`, `R` | Buộc làm mới dữ liệu |
| `Tab` | Chuyển đổi giữa các nhóm mã |

## Cấu hình

Tạo một tệp cấu hình TOML:

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

# Tài sản nắm giữ trong danh mục (tùy chọn)
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

# Nhóm mã
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Hiển thị đa tiền tệ (tùy chọn)
[currency]
display = "EUR"
convert = true

# Cảnh báo giá (tùy chọn)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Ký hiệu tắt tiền mã hóa tùy chỉnh (tùy chọn)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Định dạng mã

| Định dạng | Mô tả | Ví dụ |
|-----------|-------|-------|
| Cổ phiếu | Mã chứng khoán tiêu chuẩn | `AAPL`, `GOOGL` |
| Tiền mã hóa | Kèm hậu tố USD | `BTC-USD`, `ETH-USD` |
| Viết tắt tiền mã hóa | Tự động mở rộng thành -USD | `BTC.X` -> `BTC-USD` |
| Tiền mã hóa tự động | Các đồng tiền mã hóa phổ biến | `BTC` -> `BTC-USD` |

## Nguồn dữ liệu

Stonktop sử dụng Yahoo Finance API để lấy báo giá theo thời gian thực. Không cần khóa API.

## Phát triển

### Biên dịch

```bash
# Biên dịch bản gỡ lỗi (debug)
cargo build

# Biên dịch bản phát hành (release)
cargo build --release
```

### Kiểm thử

```bash
# Chạy toàn bộ bộ kiểm thử
cargo test

# Chạy kiểm thử kèm hiển thị đầu ra
cargo test -- --nocapture
```

### Kiểm tra mã tĩnh (Lint)

```bash
# Kiểm tra các vấn đề về chất lượng mã
cargo clippy --all-targets --all-features

# Tự động áp dụng các bản sửa lỗi
cargo clippy --fix --allow-dirty
```

### Định dạng mã nguồn

```bash
# Kiểm tra định dạng
cargo fmt -- --check

# Áp dụng định dạng
cargo fmt
```

## Đóng góp

Mọi đóng góp đều được hoan nghênh! Vui lòng gửi Pull Request.

1. Fork kho lưu trữ này
2. Tạo nhánh tính năng của bạn (`git checkout -b feature/amazing-feature`)
3. Commit các thay đổi của bạn (`git commit -m 'Add amazing feature'`)
4. Đẩy nhánh lên kho lưu trữ từ xa (`git push origin feature/amazing-feature`)
5. Mở một Pull Request

## Giấy phép

Dự án này được cấp phép theo Giấy phép MIT — xem tệp [LICENSE](../../../LICENSE) để biết thêm chi tiết.

## Lời cảm ơn

- Lấy cảm hứng từ [ticker](https://github.com/achannarasappa/ticker)
- Xây dựng bằng [ratatui](https://github.com/ratatui-org/ratatui)
- Dữ liệu từ [Yahoo Finance](https://finance.yahoo.com/)
