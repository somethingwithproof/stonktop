# Stonktop

> [English](../../../README.md)

一款類似 `top` 的終端機使用者介面（TUI），用於即時監控股票與加密貨幣價格。

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## 功能特色

- **即時價格監控**：提供股票與加密貨幣的即時報價
- **類似 `top` 的操作介面**：採用與 `top`/`htop` 相近的熟悉鍵盤操作方式
- **迷你走勢圖（Sparkline）**：於側邊欄內嵌呈現價格趨勢視覺化圖表
- **投資組合追蹤**：監控您的持股狀況並計算損益（P/L）
- **搜尋與篩選**：跨代號與名稱進行不分大小寫的即時搜尋
- **價格警示**：當價格突破或跌破設定門檻時發送桌面通知
- **匯出／匯入**：以 JSON 或 CSV 格式儲存及載入觀察清單
- **多幣別支援**：支援即時匯率換算，以任意幣別顯示價格
- **多種排序方式**：可依代號、價格、漲跌、成交量與市值排序
- **批次模式**：提供非互動式輸出以利腳本自動化整合（類似 `top -b`）
- **TOML 設定檔**：結構清晰、易於編輯的設定檔案格式
- **色彩標示**：上漲以綠色顯示，下跌以紅色顯示
- **加密貨幣簡寫**：支援使用 `BTC.X` 或直接輸入 `BTC` 代表 `BTC-USD`

## 畫面預覽

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

## 系統需求

- **Rust**：1.89 或以上版本（最低支援 Rust 版本，MSRV）
- **Cargo**：隨 Rust 安裝程式一併附帶

## 安裝方式

### 從 crates.io 安裝（建議）

```bash
cargo install stonktop
```

### 使用 cargo-binstall 安裝

若系統已安裝 [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)，可直接安裝預先建置的二進位執行檔：

```bash
cargo binstall stonktop
```

### 預先建置的二進位檔

請前往 [Releases](https://github.com/somethingwithproof/stonktop/releases) 頁面下載預先建置的二進位檔。

支援平台：
- Linux（x86_64、aarch64、musl）
- macOS（Intel、Apple Silicon）
- Windows（x86_64、aarch64）

### 從原始碼建置

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## 使用方式

### 基本用法

```bash
# 監控指定標的代號
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# 使用加密貨幣簡寫
stonktop -s AAPL,BTC,ETH,SOL
```

### 類似 top 的選項

```bash
# 設定更新間隔秒數（類似 top -d）
stonktop -s AAPL,GOOGL -d 10

# 執行 N 次更新後自動結束（類似 top -n）
stonktop -s AAPL -n 5

# 批次模式，適用於腳本處理（類似 top -b）
stonktop -s AAPL,GOOGL -b

# 安全模式：停用互動式指令
stonktop -s AAPL -S
```

### 排序選項

```bash
# 依價格排序（遞減）
stonktop -s AAPL,GOOGL -o price

# 依代號排序（遞增）
stonktop -s AAPL,GOOGL -o symbol -r

# 可用排序欄位：symbol、name、price、change、change-percent、volume、market-cap
```

### 設定檔

```bash
# 指定特定的設定檔路徑
stonktop -c ~/.config/stonktop/custom.toml

# 或將設定檔放置於預設位置：
# - Linux：~/.config/stonktop/config.toml
# - macOS：~/Library/Application Support/stonktop/config.toml
# - Windows：%APPDATA%\stonktop\config.toml
```

## 指令列選項

| 選項 | 簡寫 | 說明 |
|------|------|------|
| `--symbols` | `-s` | 以逗號分隔的監控標的代號清單 |
| `--delay` | `-d` | 更新間隔秒數（預設：5） |
| `--iterations` | `-n` | 更新次數（0 表示無限次） |
| `--batch` | `-b` | 批次模式：非互動式輸出 |
| `--secure` | `-S` | 安全模式：停用互動式指令 |
| `--config` | `-c` | 設定檔路徑 |
| `--sort` | `-o` | 初始排序欄位 |
| `--reverse` | `-r` | 反轉排序順序 |
| `--holdings` | `-H` | 顯示持股／投資組合檢視 |
| `--currency` | | 顯示幣別（預設：USD） |
| `--timeout` | | API 逾時秒數（預設：10） |
| `--format` | | 批次模式的輸出格式：table、json、csv |
| `--color` | | 色彩輸出模式：auto、always、never |
| `--verbose` | `-v` | 顯示詳細輸出 |
| `--init` | | 產生範例設定檔 |
| `--force` | | 強制覆寫現有設定檔（搭配 `--init` 使用） |
| `--lang` | | 顯示語言（`STONKTOP_LANG`，預設自動偵測 `LC_ALL`／`LC_MESSAGES`／`LANG`） |
| `--help` | | 顯示說明訊息 |
| `--version` | `-V` | 顯示版本資訊 |

## 支援語言

Stonktop 會依序從 `STONKTOP_LANG`、`LC_ALL`、`LC_MESSAGES` 與 `LANG` 自動偵測顯示語言，亦可透過 `--lang <LANG>` 手動指定。全部 48 個支援語系皆具備完整的 CLI 與 TUI 在地化翻譯（包含阿拉伯語、波斯語、希伯來語與烏爾都語的由右至左對齊排版）及在地化說明文件：

| 代碼 | 語言 | README | 代碼 | 語言 | README | 代碼 | 語言 | README |
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

## 鍵盤快速鍵

| 按鍵 | 功能 |
|------|------|
| `q`, `Esc` | 離開程式 |
| `h`, `?` | 切換說明視窗 |
| `↑`, `k` | 向上移動 |
| `↓`, `j` | 向下移動 |
| `g`, `Home` | 移至頂端 |
| `G`, `End` | 移至底端 |
| `PgUp` | 向上翻頁 |
| `PgDn` | 向下翻頁 |
| `s` | 循環切換排序欄位 |
| `r` | 反轉排序順序 |
| `1-7` | 依指定欄位排序 |
| `Enter` | 顯示所選標的之詳細資訊彈出視窗 |
| `/` | 搜尋／篩選標的代號 |
| `a` | 新增標的至觀察清單 |
| `d` | 移除所選標的 |
| `S` | 切換迷你走勢圖側邊欄 |
| `e` | 將觀察清單匯出為 JSON |
| `H` | 切換持股檢視 |
| `f` | 切換基本面資訊 |
| `Space`, `R` | 強制重新整理 |
| `Tab` | 循環切換標的群組 |

## 設定說明

建立一份 TOML 設定檔：

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

# 投資組合持股（選用）
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

# 標的群組
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# 多幣別顯示（選用）
[currency]
display = "EUR"
convert = true

# 價格警示（選用）
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# 自訂加密貨幣簡寫（選用）
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## 標的代號格式

| 格式 | 說明 | 範例 |
|------|------|------|
| 股票 | 標準股票代號 | `AAPL`、`GOOGL` |
| 加密貨幣 | 加上 USD 後綴 | `BTC-USD`、`ETH-USD` |
| 加密貨幣簡寫 | 自動展開為 -USD | `BTC.X` -> `BTC-USD` |
| 常見加密貨幣自動轉換 | 常用加密貨幣簡稱 | `BTC` -> `BTC-USD` |

## 資料來源

Stonktop 透過 Yahoo Finance API 擷取即時報價，無需設定 API 金鑰。

## 開發指南

### 建置

```bash
# 除錯模式建置
cargo build

# 發行模式建置
cargo build --release
```

### 測試

```bash
# 執行所有測試
cargo test

# 執行測試並顯示標準輸出
cargo test -- --nocapture
```

### 靜態分析（Lint）

```bash
# 檢查程式碼靜態分析問題
cargo clippy --all-targets --all-features

# 自動套用修正建議
cargo clippy --fix --allow-dirty
```

### 程式碼格式化

```bash
# 檢查格式是否符合規範
cargo fmt -- --check

# 套用程式碼格式化
cargo fmt
```

## 參與貢獻

歡迎各界參與貢獻！請隨時提交 Pull Request。

1. Fork 本專案儲存庫
2. 建立您的功能分支（`git checkout -b feature/amazing-feature`）
3. 提交您的變更（`git commit -m 'Add amazing feature'`）
4. 將分支推送至遠端（`git push origin feature/amazing-feature`）
5. 開啟 Pull Request

## 授權條款

本專案採用 MIT 授權條款釋出，詳細內容請參閱 [LICENSE](../../../LICENSE) 檔案。

## 致謝

- 靈感源自 [ticker](https://github.com/achannarasappa/ticker)
- 使用 [ratatui](https://github.com/ratatui-org/ratatui) 建構
- 報價資料來源：[Yahoo Finance](https://finance.yahoo.com/)
