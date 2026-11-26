# Stonktop

> [English](../../../README.md)

株式および暗号資産の価格をリアルタイムで監視するための、`top` 風ターミナル UI です。

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## 主な機能

- **リアルタイム価格監視**: 株式・暗号資産のリアルタイム相場を取得
- **`top` 風インターフェース**: `top`／`htop` に近い直感的なキーボード操作
- **スパークラインチャート**: サイドバーに価格推移のミニグラフを表示
- **ポートフォリオ管理**: 保有資産と損益（P/L）の計算・監視
- **検索・フィルタリング**: シンボル名および銘柄名に対する大文字・小文字を区別しない検索
- **価格アラート**: 設定したしきい値を価格が超えた際にデスクトップ通知を表示
- **エクスポート／インポート**: ウォッチリストを JSON または CSV 形式で保存・読み込み
- **多通貨対応**: リアルタイムの為替レートを用いて任意の通貨で価格を表示
- **多彩な並び替え**: シンボル、価格、騰落額、出来高、時価総額によるソート
- **バッチモード**: スクリプト連携向けの非対話出力（`top -b` 相当）
- **TOML 設定ファイル**: 編集しやすい TOML 形式での設定管理
- **色分け表示**: 上昇を緑、下落を赤でわかりやすく表示
- **暗号資産ショートカット**: `BTC-USD` の代わりに `BTC.X` または `BTC` と省略指定可能

## スクリーンショット

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

## 動作要件

- **Rust**: バージョン 1.89 以上（MSRV）
- **Cargo**: Rust のインストールに同梱

## インストール

### crates.io からインストール（推奨）

```bash
cargo install stonktop
```

### cargo-binstall を使用

[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) がインストールされている場合、ビルド済みバイナリをインストールできます。

```bash
cargo binstall stonktop
```

### ビルド済みバイナリ

[Releases](https://github.com/somethingwithproof/stonktop/releases) ページからビルド済みバイナリをダウンロードできます。

対応プラットフォーム:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### ソースからビルド

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## 使い方

### 基本的な使い方

```bash
# 指定したシンボルを監視
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# 暗号資産ショートカットを使用
stonktop -s AAPL,BTC,ETH,SOL
```

### `top` 風オプション

```bash
# 更新間隔を設定（top -d と同等）
stonktop -s AAPL,GOOGL -d 10

# N 回更新して終了（top -n と同等）
stonktop -s AAPL -n 5

# スクリプト向けバッチモード（top -b と同等）
stonktop -s AAPL,GOOGL -b

# セキュアモード（対話コマンドを無効化）
stonktop -s AAPL -S
```

### ソートオプション

```bash
# 価格順に並び替え（降順）
stonktop -s AAPL,GOOGL -o price

# シンボル順に並び替え（昇順）
stonktop -s AAPL,GOOGL -o symbol -r

# 指定可能なソート項目: symbol, name, price, change, change-percent, volume, market-cap
```

### 設定ファイル

```bash
# 指定した設定ファイルを使用
stonktop -c ~/.config/stonktop/custom.toml

# またはデフォルトのパスに設定ファイルを配置:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## コマンドラインオプション

| オプション | 短縮形 | 説明 |
|------------|--------|------|
| `--symbols` | `-s` | 監視するシンボルのカンマ区切りリスト |
| `--delay` | `-d` | 更新間隔（秒、デフォルト: 5） |
| `--iterations` | `-n` | 更新回数（0 = 無制限） |
| `--batch` | `-b` | バッチモード（非対話出力） |
| `--secure` | `-S` | セキュアモード（対話コマンドを無効化） |
| `--config` | `-c` | 設定ファイルのパス |
| `--sort` | `-o` | 初期ソート項目 |
| `--reverse` | `-r` | ソート順を反転 |
| `--holdings` | `-H` | 保有資産／ポートフォリオ表示を有効化 |
| `--currency` | | 表示通貨（デフォルト: USD） |
| `--timeout` | | API タイムアウト（秒、デフォルト: 10） |
| `--format` | | バッチモードの出力形式: table, json, csv |
| `--color` | | カラー出力: auto, always, never |
| `--verbose` | `-v` | 詳細出力 |
| `--init` | | サンプル設定ファイルを生成 |
| `--force` | | 既存の設定ファイルを強制上書き（`--init` と併用） |
| `--lang` | | 表示言語（`STONKTOP_LANG`、デフォルトは `LC_ALL`/`LC_MESSAGES`/`LANG` から自動判定） |
| `--help` | | ヘルプメッセージを表示 |
| `--version` | `-V` | バージョン情報を表示 |

## 対応言語

Stonktop は `STONKTOP_LANG`、`LC_ALL`、`LC_MESSAGES`、`LANG` の各環境変数から表示言語を自動検出するほか、`--lang <LANG>` による明示的な指定にも対応しています。対応する全 45 ロケールで CLI および TUI の完全な翻訳（アラビア語、ペルシア語、ヘブライ語、ウルドゥー語の右から左へのレイアウト対応を含む）とローカライズ済みドキュメントを提供しています。

| コード | 言語 | README | コード | 言語 | README | コード | 言語 | README |
|--------|------|--------|--------|------|--------|--------|------|--------|
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

## キーボードショートカット

| キー | 操作 |
|------|------|
| `q`, `Esc` | 終了 |
| `h`, `?` | ヘルプの表示切り替え |
| `↑`, `k` | 上へ移動 |
| `↓`, `j` | 下へ移動 |
| `g`, `Home` | 先頭へ移動 |
| `G`, `End` | 末尾へ移動 |
| `PgUp` | 1 ページ上へ |
| `PgDn` | 1 ページ下へ |
| `s` | ソート項目を切り替え |
| `r` | ソート順を反転 |
| `1-7` | 列番号でソート |
| `Enter` | 選択中のシンボルの詳細ポップアップを表示 |
| `/` | シンボルを検索／フィルタリング |
| `a` | ウォッチリストにシンボルを追加 |
| `d` | 選択中のシンボルを削除 |
| `S` | スパークラインチャートのサイドバー表示を切り替え |
| `e` | ウォッチリストを JSON にエクスポート |
| `H` | 保有資産表示を切り替え |
| `f` | ファンダメンタル指標の表示を切り替え |
| `Space`, `R` | 強制更新 |
| `Tab` | シンボルグループを切り替え |

## 設定

TOML 形式の設定ファイルを作成します。

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

# ポートフォリオの保有資産（任意）
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

# シンボルグループ
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# 多通貨表示（任意）
[currency]
display = "EUR"
convert = true

# 価格アラート（任意）
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# カスタム暗号資産ショートカット（任意）
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## シンボル形式

| 形式 | 説明 | 例 |
|------|------|----|
| 株式 | 標準ティッカー | `AAPL`, `GOOGL` |
| 暗号資産 | USD サフィックス付き | `BTC-USD`, `ETH-USD` |
| 暗号資産短縮表記 | `-USD` に展開 | `BTC.X` -> `BTC-USD` |
| 暗号資産自動補完 | 主要な暗号資産 | `BTC` -> `BTC-USD` |

## データソース

Stonktop はリアルタイム相場の取得に Yahoo Finance API を使用します。API キーは不要です。

## 開発

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト

```bash
# すべてのテストを実行
cargo test

# 標準出力を表示してテストを実行
cargo test -- --nocapture
```

### リント

```bash
# リント警告をチェック
cargo clippy --all-targets --all-features

# 自動修正を適用
cargo clippy --fix --allow-dirty
```

### フォーマット

```bash
# フォーマットをチェック
cargo fmt -- --check

# フォーマットを適用
cargo fmt
```

## コントリビューション

コントリビューションを歓迎します。ぜひ Pull Request をお送りください。

1. リポジトリをフォークする
2. 機能ブランチを作成する（`git checkout -b feature/amazing-feature`）
3. 変更をコミットする（`git commit -m 'Add amazing feature'`）
4. ブランチをプッシュする（`git push origin feature/amazing-feature`）
5. Pull Request を作成する

## ライセンス

本プロジェクトは MIT ライセンスのもとで公開されています。詳細は [LICENSE](../../../LICENSE) を参照してください。

## 謝辞

- [ticker](https://github.com/achannarasappa/ticker) に着想を得ています
- [ratatui](https://github.com/ratatui-org/ratatui) を使用して構築されています
- データ提供: [Yahoo Finance](https://finance.yahoo.com/)
