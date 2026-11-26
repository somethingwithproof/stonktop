# Stonktop

> [English](../../../README.md)

一款类似 `top` 的终端用户界面工具，用于实时监控股票和加密货币价格。

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## 功能特性

- **实时价格监控**：获取股票与加密货币的实时行情报价
- **类 `top` 交互界面**：与 `top`/`htop` 相似的熟悉键盘操作方式
- **迷你走势图（Sparkline）**：在侧边栏中直观呈现价格趋势变化
- **投资组合追踪**：监控持仓资产并实时计算盈亏（P/L）
- **搜索与筛选**：支持按代码和名称进行不区分大小写的检索
- **价格预警**：价格突破预设阈值时发送桌面通知
- **导出与导入**：支持将自选列表保存或加载为 JSON 或 CSV 格式
- **多币种显示**：结合实时汇率，以任意指定币种显示价格
- **多种排序方式**：支持按代码、价格、涨跌额、成交量与市值排序
- **批处理模式**：面向脚本自动化的非交互式输出（类似 `top -b`）
- **TOML 配置文件**：简洁直观、易于编辑的配置格式
- **涨跌色彩标识**：绿色表示上涨，红色表示下跌
- **加密货币快捷代码**：可使用 `BTC.X` 或 `BTC` 直接指代 `BTC-USD`

## 界面预览

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

## 环境要求

- **Rust**：1.89 及以上版本（MSRV）
- **Cargo**：随 Rust 安装包自动附带

## 安装

### 通过 crates.io 安装（推荐）

```bash
cargo install stonktop
```

### 使用 cargo-binstall 安装

如果已安装 [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)，可直接安装预编译二进制文件：

```bash
cargo binstall stonktop
```

### 预编译二进制文件

可从 [Releases](https://github.com/somethingwithproof/stonktop/releases) 页面下载预编译二进制文件。

支持的平台：
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### 从源码编译

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## 使用说明

### 基本用法

```bash
# 监控指定代码
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# 使用加密货币快捷代码
stonktop -s AAPL,BTC,ETH,SOL
```

### 类 `top` 选项

```bash
# 设置刷新间隔（类似 top -d）
stonktop -s AAPL,GOOGL -d 10

# 运行 N 次刷新后退出（类似 top -n）
stonktop -s AAPL -n 5

# 用于脚本的批处理模式（类似 top -b）
stonktop -s AAPL,GOOGL -b

# 安全模式——禁用交互式命令
stonktop -s AAPL -S
```

### 排序选项

```bash
# 按价格排序（降序）
stonktop -s AAPL,GOOGL -o price

# 按代码排序（升序）
stonktop -s AAPL,GOOGL -o symbol -r

# 可用的排序字段: symbol, name, price, change, change-percent, volume, market-cap
```

### 配置文件

```bash
# 使用指定的配置文件
stonktop -c ~/.config/stonktop/custom.toml

# 或将配置文件放置在默认位置：
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## 命令行选项

| 选项 | 简写 | 说明 |
|------|------|------|
| `--symbols` | `-s` | 以逗号分隔的监控代码列表 |
| `--delay` | `-d` | 刷新间隔，单位为秒（默认：5） |
| `--iterations` | `-n` | 刷新次数（0 = 无限循环） |
| `--batch` | `-b` | 批处理模式——非交互式输出 |
| `--secure` | `-S` | 安全模式——禁用交互式命令 |
| `--config` | `-c` | 配置文件路径 |
| `--sort` | `-o` | 初始排序字段 |
| `--reverse` | `-r` | 反转排序顺序 |
| `--holdings` | `-H` | 显示持仓/投资组合视图 |
| `--currency` | | 显示币种（默认：USD） |
| `--timeout` | | API 超时时间，单位为秒（默认：10） |
| `--format` | | 批处理模式输出格式：table, json, csv |
| `--color` | | 彩色输出：auto, always, never |
| `--verbose` | `-v` | 详细输出 |
| `--init` | | 生成示例配置文件 |
| `--force` | | 强制覆盖已有配置文件（与 `--init` 配合使用） |
| `--lang` | | 显示语言（`STONKTOP_LANG`，默认从 `LC_ALL`/`LC_MESSAGES`/`LANG` 自动检测） |
| `--help` | | 显示帮助信息 |
| `--version` | `-V` | 显示版本号 |

## 支持的语言

Stonktop 会从 `STONKTOP_LANG`、`LC_ALL`、`LC_MESSAGES` 和 `LANG` 自动检测显示语言，也可通过 `--lang <LANG>` 手动指定。全部 48 种受支持的语言区域均包含完整的 CLI 与 TUI 翻译（包括对阿拉伯语、波斯语、希伯来语和乌尔都语的从右到左排版对齐支持）以及本地化文档：

| 代码 | 语言 | README | 代码 | 语言 | README | 代码 | 语言 | README |
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

## 快捷键

| 按键 | 功能 |
|------|------|
| `q`, `Esc` | 退出 |
| `h`, `?` | 切换帮助界面 |
| `↑`, `k` | 向上移动 |
| `↓`, `j` | 向下移动 |
| `g`, `Home` | 跳至顶部 |
| `G`, `End` | 跳至底部 |
| `PgUp` | 向上翻页 |
| `PgDn` | 向下翻页 |
| `s` | 切换排序字段 |
| `r` | 反转排序顺序 |
| `1-7` | 按指定列排序 |
| `Enter` | 显示所选代码的详情弹窗 |
| `/` | 搜索/筛选代码 |
| `a` | 向自选列表添加代码 |
| `d` | 移除所选代码 |
| `S` | 切换迷你走势图侧边栏 |
| `e` | 将自选列表导出为 JSON |
| `H` | 切换持仓视图 |
| `f` | 切换基本面数据显示 |
| `Space`, `R` | 强制刷新 |
| `Tab` | 切换代码分组 |

## 配置说明

创建一个 TOML 格式的配置文件：

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

# 投资组合持仓（可选）
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

# 代码分组
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# 多币种显示（可选）
[currency]
display = "EUR"
convert = true

# 价格预警（可选）
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# 自定义加密货币快捷代码（可选）
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## 代码格式

| 格式 | 说明 | 示例 |
|------|------|------|
| 股票 | 标准股票代码 | `AAPL`, `GOOGL` |
| 加密货币 | 带 USD 后缀 | `BTC-USD`, `ETH-USD` |
| 加密货币简写 | 自动展开为 `-USD` | `BTC.X` -> `BTC-USD` |
| 加密货币自动识别 | 常见加密货币 | `BTC` -> `BTC-USD` |

## 数据来源

Stonktop 使用 Yahoo Finance API 获取实时行情数据，无需申请 API 密钥。

## 开发指南

### 构建

```bash
# 调试版本构建
cargo build

# 发布版本构建
cargo build --release
```

### 测试

```bash
# 运行全部测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 检查代码规范问题
cargo clippy --all-targets --all-features

# 自动应用修复建议
cargo clippy --fix --allow-dirty
```

### 代码格式化

```bash
# 检查代码格式
cargo fmt -- --check

# 应用代码格式化
cargo fmt
```

## 参与贡献

欢迎参与贡献！欢迎随时提交 Pull Request。

1. Fork 本仓库
2. 创建功能分支（`git checkout -b feature/amazing-feature`）
3. 提交更改（`git commit -m 'Add amazing feature'`）
4. 推送至分支（`git push origin feature/amazing-feature`）
5. 创建 Pull Request

## 许可证

本项目基于 MIT 许可证开源，详情请参阅 [LICENSE](../../../LICENSE) 文件。

## 致谢

- 灵感来源于 [ticker](https://github.com/achannarasappa/ticker)
- 基于 [ratatui](https://github.com/ratatui-org/ratatui) 构建
- 行情数据来自 [Yahoo Finance](https://finance.yahoo.com/)
