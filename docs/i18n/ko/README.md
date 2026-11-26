# Stonktop

> [English](../../../README.md)

주식 및 암호화폐 시세를 실시간으로 모니터링하기 위한 `top` 스타일의 터미널 UI입니다.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## 주요 기능

- **실시간 시세 모니터링**: 주식 및 암호화폐 실시간 가격 조회
- **`top` 스타일 인터페이스**: `top`/`htop`과 유사한 익숙한 키보드 조작
- **스파크라인 차트**: 사이드바에 인라인 가격 추세 시각화 제공
- **포트폴리오 추적**: 손익(P/L) 계산을 포함한 보유 자산 모니터링
- **검색 및 필터링**: 심볼과 종목명에 대한 대소문자 구분 없는 검색
- **가격 알림**: 가격이 설정된 임계값을 통과할 때 데스크톱 알림 전송
- **내보내기/가져오기**: 관심 목록을 JSON 또는 CSV 형식으로 저장 및 불러오기
- **다중 통화 지원**: 실시간 환율을 적용하여 원하는 통화로 가격 표시
- **다양한 정렬 옵션**: 심볼, 가격, 변동액, 거래량, 시가총액 기준 정렬
- **배치 모드**: 스크립트 자동화를 위한 비대화형 출력(`top -b`와 유사)
- **TOML 설정**: 편집하기 쉬운 TOML 설정 파일 지원
- **색상 구분**: 상승은 녹색, 하락은 빨간색으로 표시
- **암호화폐 단축 표기**: `BTC-USD` 대신 `BTC.X` 또는 `BTC` 사용 가능

## 스크린샷

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

## 사전 요구 사항

- **Rust**: 버전 1.89 이상(MSRV)
- **Cargo**: Rust 설치 시 함께 포함됨

## 설치

### crates.io에서 설치(권장)

```bash
cargo install stonktop
```

### cargo-binstall 사용

[cargo-binstall](https://github.com/cargo-bins/cargo-binstall)이 설치되어 있는 경우 사전 빌드된 바이너리를 설치할 수 있습니다.

```bash
cargo binstall stonktop
```

### 사전 빌드된 바이너리

[Releases](https://github.com/somethingwithproof/stonktop/releases) 페이지에서 사전 빌드된 바이너리를 다운로드할 수 있습니다.

지원 플랫폼:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### 소스에서 빌드

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## 사용법

### 기본 사용법

```bash
# 특정 심볼 모니터링
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# 암호화폐 단축 표기 사용
stonktop -s AAPL,BTC,ETH,SOL
```

### `top` 스타일 옵션

```bash
# 갱신 간격 설정 (top -d와 동일)
stonktop -s AAPL,GOOGL -d 10

# N회 반복 후 종료 (top -n과 동일)
stonktop -s AAPL -n 5

# 스크립트용 배치 모드 (top -b와 동일)
stonktop -s AAPL,GOOGL -b

# 보안 모드 - 대화형 명령 비활성화
stonktop -s AAPL -S
```

### 정렬 옵션

```bash
# 가격 기준 정렬 (내림차순)
stonktop -s AAPL,GOOGL -o price

# 심볼 기준 정렬 (오름차순)
stonktop -s AAPL,GOOGL -o symbol -r

# 사용 가능한 정렬 필드: symbol, name, price, change, change-percent, volume, market-cap
```

### 설정 파일

```bash
# 지정된 설정 파일 사용
stonktop -c ~/.config/stonktop/custom.toml

# 또는 기본 위치에 설정 파일 배치:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## 명령줄 옵션

| 옵션 | 단축 | 설명 |
|------|------|------|
| `--symbols` | `-s` | 모니터링할 심볼의 쉼표 구분 목록 |
| `--delay` | `-d` | 초 단위 갱신 간격 (기본값: 5) |
| `--iterations` | `-n` | 반복 횟수 (0 = 무한) |
| `--batch` | `-b` | 배치 모드 - 비대화형 출력 |
| `--secure` | `-S` | 보안 모드 - 대화형 명령 비활성화 |
| `--config` | `-c` | 설정 파일 경로 |
| `--sort` | `-o` | 초기 정렬 필드 |
| `--reverse` | `-r` | 정렬 순서 반전 |
| `--holdings` | `-H` | 보유 자산/포트폴리오 보기 표시 |
| `--currency` | | 표시 통화 (기본값: USD) |
| `--timeout` | | 초 단위 API 타임아웃 (기본값: 10) |
| `--format` | | 배치 모드 출력 형식: table, json, csv |
| `--color` | | 색상 출력: auto, always, never |
| `--verbose` | `-v` | 상세 출력 |
| `--init` | | 샘플 설정 파일 생성 |
| `--force` | | 기존 설정 파일 강제 덮어쓰기 (`--init`와 함께 사용) |
| `--lang` | | 표시 언어 (`STONKTOP_LANG`, 기본값: `LC_ALL`/`LC_MESSAGES`/`LANG`에서 자동 감지) |
| `--help` | | 도움말 메시지 표시 |
| `--version` | `-V` | 버전 표시 |

## 지원 언어

Stonktop은 `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, `LANG` 환경 변수에서 표시 언어를 자동 감지하거나 `--lang <LANG>` 옵션을 지원합니다. 지원되는 48개 로캘 모두 완전한 CLI 및 TUI 번역(아랍어, 페르시아어, 히브리어, 우르두어의 오른쪽에서 왼쪽 정렬 포함)과 현지화된 문서를 제공합니다.

| 코드 | 언어 | README | 코드 | 언어 | README | 코드 | 언어 | README |
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

## 키보드 단축키

| 키 | 동작 |
|----|------|
| `q`, `Esc` | 종료 |
| `h`, `?` | 도움말 전환 |
| `↑`, `k` | 위로 이동 |
| `↓`, `j` | 아래로 이동 |
| `g`, `Home` | 맨 위로 이동 |
| `G`, `End` | 맨 아래로 이동 |
| `PgUp` | 한 페이지 위로 |
| `PgDn` | 한 페이지 아래로 |
| `s` | 정렬 필드 순환 |
| `r` | 정렬 순서 반전 |
| `1-7` | 열 기준 정렬 |
| `Enter` | 선택한 심볼의 상세 팝업 표시 |
| `/` | 심볼 검색/필터링 |
| `a` | 관심 목록에 심볼 추가 |
| `d` | 선택한 심볼 제거 |
| `S` | 스파크라인 차트 사이드바 전환 |
| `e` | 관심 목록을 JSON으로 내보내기 |
| `H` | 보유 자산 보기 전환 |
| `f` | 기본적 지표(펀더멘털) 표시 전환 |
| `Space`, `R` | 강제 새로고침 |
| `Tab` | 심볼 그룹 순환 |

## 설정

TOML 설정 파일을 생성합니다.

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

# 포트폴리오 보유 자산 (선택 사항)
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

# 심볼 그룹
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# 다중 통화 표시 (선택 사항)
[currency]
display = "EUR"
convert = true

# 가격 알림 (선택 사항)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# 사용자 지정 암호화폐 단축 표기 (선택 사항)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## 심볼 형식

| 형식 | 설명 | 예시 |
|------|------|------|
| 주식 | 표준 티커 | `AAPL`, `GOOGL` |
| 암호화폐 | USD 접미사 포함 | `BTC-USD`, `ETH-USD` |
| 암호화폐 축약형 | `-USD`로 확장 | `BTC.X` -> `BTC-USD` |
| 암호화폐 자동 변환 | 주요 암호화폐 | `BTC` -> `BTC-USD` |

## 데이터 출처

Stonktop은 실시간 시세를 가져오기 위해 Yahoo Finance API를 사용합니다. 별도의 API 키가 필요하지 않습니다.

## 개발

### 빌드

```bash
# 디버그 빌드
cargo build

# 릴리스 빌드
cargo build --release
```

### 테스트

```bash
# 모든 테스트 실행
cargo test

# 출력을 포함하여 테스트 실행
cargo test -- --nocapture
```

### 린트

```bash
# 린트 문제 검사
cargo clippy --all-targets --all-features

# 자동 수정 적용
cargo clippy --fix --allow-dirty
```

### 포맷

```bash
# 코드 포맷 검사
cargo fmt -- --check

# 코드 포맷 적용
cargo fmt
```

## 기여

기여를 환영합니다. Pull Request를 자유롭게 제출해 주세요.

1. 저장소를 포크(Fork)합니다
2. 기능 브랜치를 생성합니다 (`git checkout -b feature/amazing-feature`)
3. 변경 사항을 커밋합니다 (`git commit -m 'Add amazing feature'`)
4. 브랜치를 원격 저장소에 푸시합니다 (`git push origin feature/amazing-feature`)
5. Pull Request를 엽니다

## 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](../../../LICENSE) 파일을 참조하세요.

## 감사의 말

- [ticker](https://github.com/achannarasappa/ticker)에서 영감을 받았습니다
- [ratatui](https://github.com/ratatui-org/ratatui)로 제작되었습니다
- 데이터 제공: [Yahoo Finance](https://finance.yahoo.com/)
