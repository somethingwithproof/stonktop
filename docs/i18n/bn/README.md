# Stonktop

> [English](../../../README.md)

রিয়েল-টাইমে শেয়ার এবং ক্রিপ্টোকারেন্সির মূল্য পর্যবেক্ষণের জন্য `top`-সদৃশ একটি টার্মিনাল ইউজার ইন্টারফেস।

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## বৈশিষ্ট্যসমূহ

- **রিয়েল-টাইম মূল্য পর্যবেক্ষণ**: শেয়ার এবং ক্রিপ্টোকারেন্সির তাৎক্ষণিক বাজারদর
- **টপ-সদৃশ ইন্টারফেস**: `top`/`htop`-এর মতো পরিচিত কিবোর্ড নিয়ন্ত্রণ ব্যবস্থা
- **স্পার্কলাইন চার্ট**: সাইডবারে ইনলাইন মূল্য প্রবণতার দৃশ্যমান উপস্থাপনা
- **পোর্টফোলিও ট্র্যাকিং**: লাভ-ক্ষতির (পি/এল) হিসাবসহ আপনার বিনিয়োগ পর্যবেক্ষণ
- **অনুসন্ধান ও ফিল্টার**: সিম্বল এবং নামজুড়ে কেস-ইনসেন্সিটিভ অনুসন্ধান
- **মূল্য সতর্কতা**: মূল্য নির্ধারিত সীমা অতিক্রম করলে ডেস্কটপ নোটিফিকেশন
- **এক্সপোর্ট/ইমপোর্ট**: জেসন বা সিএসভি ফরম্যাটে ওয়াচলিস্ট সংরক্ষণ এবং লোড করা
- **বহু-মুদ্রা সমর্থন**: তাৎক্ষণিক বিনিময় হারসহ যেকোনো মুদ্রায় মূল্য প্রদর্শন
- **একাধিক সাজানোর বিকল্প**: সিম্বল, মূল্য, পরিবর্তন, ভলিউম এবং বাজার মূলধন অনুসারে সাজানো
- **ব্যাচ মোড**: স্ক্রিপ্টিংয়ের জন্য নন-ইন্টারঅ্যাক্টিভ আউটপুট (`top -b`-এর মতো)
- **টিওএমএল কনফিগারেশন**: সহজে সম্পাদনাযোগ্য কনফিগারেশন ফাইল
- **রঙের সংকেত**: লাভের জন্য সবুজ এবং ক্ষতির জন্য লাল রঙ
- **ক্রিপ্টো শর্টকাট**: `BTC-USD`-এর পরিবর্তে `BTC.X` অথবা কেবল `BTC` ব্যবহারের সুবিধা

## স্ক্রিনশট

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

## পূর্বশর্ত

- **রাস্ট**: সংস্করণ ১.৮৯ বা তদূর্ধ্ব (সর্বনিম্ন সমর্থিত রাস্ট সংস্করণ)
- **কার্গো**: রাস্ট ইনস্টলেশনের সাথে অন্তর্ভুক্ত

## ইনস্টলেশন

### crates.io থেকে (প্রস্তাবিত)

```bash
cargo install stonktop
```

### cargo-binstall ব্যবহার করে

আপনার সিস্টেমে [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) ইনস্টল করা থাকলে পূর্ব-নির্মিত বাইনারি ইনস্টল করতে পারেন:

```bash
cargo binstall stonktop
```

### পূর্ব-নির্মিত বাইনারি

[Releases](https://github.com/somethingwithproof/stonktop/releases) পৃষ্ঠা থেকে পূর্ব-নির্মিত বাইনারি ডাউনলোড করুন।

উপলব্ধ প্ল্যাটফর্মসমূহ:
- লিনাক্স (x86_64, aarch64, musl)
- ম্যাকওএস (ইন্টেল, অ্যাপল সিলিকন)
- উইন্ডোজ (x86_64, aarch64)

### সোর্স কোড থেকে

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## ব্যবহারবিধি

### মৌলিক ব্যবহার

```bash
# নির্দিষ্ট সিম্বলসমূহ পর্যবেক্ষণ করুন
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# ক্রিপ্টো শর্টকাটসহ
stonktop -s AAPL,BTC,ETH,SOL
```

### টপ-সদৃশ বিকল্পসমূহ

```bash
# রিফ্রেশ বিরতি নির্ধারণ করুন (top -d এর মতো)
stonktop -s AAPL,GOOGL -d 10

# নির্ধারিত সংখ্যক চক্র চালিয়ে প্রস্থান করুন (top -n এর মতো)
stonktop -s AAPL -n 5

# স্ক্রিপ্টিংয়ের জন্য ব্যাচ মোড (top -b এর মতো)
stonktop -s AAPL,GOOGL -b

# নিরাপদ মোড - ইন্টারঅ্যাক্টিভ কমান্ড নিষ্ক্রিয় করুন
stonktop -s AAPL -S
```

### সাজানোর বিকল্পসমূহ

```bash
# মূল্য অনুসারে সাজান (অবতরণ ক্রমে)
stonktop -s AAPL,GOOGL -o price

# সিম্বল অনুসারে সাজান (আরোহণ ক্রমে)
stonktop -s AAPL,GOOGL -o symbol -r

# উপলব্ধ সাজানোর ক্ষেত্রসমূহ: symbol, name, price, change, change-percent, volume, market-cap
```

### কনফিগারেশন ফাইল

```bash
# নির্দিষ্ট কনফিগারেশন ফাইল ব্যবহার করুন
stonktop -c ~/.config/stonktop/custom.toml

# অথবা ডিফল্ট অবস্থানে কনফিগারেশন ফাইল রাখুন:
# - লিনাক্স: ~/.config/stonktop/config.toml
# - ম্যাকওএস: ~/Library/Application Support/stonktop/config.toml
# - উইন্ডোজ: %APPDATA%\stonktop\config.toml
```

## কমান্ড লাইন বিকল্পসমূহ

| বিকল্প | সংক্ষিপ্ত | বিবরণ |
|--------|-----------|-------|
| `--symbols` | `-s` | পর্যবেক্ষণের জন্য কমা দ্বারা পৃথক করা সিম্বলের তালিকা |
| `--delay` | `-d` | সেকেন্ডে রিফ্রেশ বিরতি (ডিফল্ট: ৫) |
| `--iterations` | `-n` | চক্রের সংখ্যা (০ = অসীম) |
| `--batch` | `-b` | ব্যাচ মোড - নন-ইন্টারঅ্যাক্টিভ আউটপুট |
| `--secure` | `-S` | নিরাপদ মোড - ইন্টারঅ্যাক্টিভ কমান্ড নিষ্ক্রিয় করে |
| `--config` | `-c` | কনফিগারেশন ফাইলের পাথ |
| `--sort` | `-o` | প্রাথমিক সাজানোর ক্ষেত্র |
| `--reverse` | `-r` | সাজানোর ক্রম বিপরীত করে |
| `--holdings` | `-H` | হোল্ডিংস/পোর্টফোলিও ভিউ প্রদর্শন করে |
| `--currency` | | প্রদর্শনের মুদ্রা (ডিফল্ট: USD) |
| `--timeout` | | সেকেন্ডে এপিআই টাইমআউট (ডিফল্ট: ১০) |
| `--format` | | ব্যাচ মোডের জন্য আউটপুট ফরম্যাট: table, json, csv |
| `--color` | | রঙিন আউটপুট: auto, always, never |
| `--verbose` | `-v` | বিস্তারিত আউটপুট |
| `--init` | | একটি নমুনা কনফিগারেশন ফাইল তৈরি করে |
| `--force` | | বিদ্যমান কনফিগারেশন জোরপূর্বক প্রতিস্থাপন করে (`--init`-এর সাথে) |
| `--lang` | | প্রদর্শনের ভাষা (`STONKTOP_LANG`, ডিফল্ট: `LC_ALL`/`LC_MESSAGES`/`LANG` থেকে স্বয়ংক্রিয়) |
| `--help` | | সহায়তা বার্তা প্রদর্শন করে |
| `--version` | `-V` | সংস্করণ প্রদর্শন করে |

## সমর্থিত ভাষাসমূহ

স্টঙ্কটপ `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES`, এবং `LANG` থেকে প্রদর্শনের ভাষা শনাক্ত করে অথবা `--lang <LANG>` গ্রহণ করে। সমর্থিত সকল ৪৫টি লোকেলে সম্পূর্ণ কমান্ড লাইন ও টার্মিনাল ইউজার ইন্টারফেস অনুবাদ (আরবি, ফারসি, হিব্রু এবং উর্দুর জন্য ডান-থেকে-বাম বিন্যাসসহ) এবং স্থানীয়কৃত নথিপত্র অন্তর্ভুক্ত রয়েছে:

| কোড | ভাষা | README | কোড | ভাষা | README | কোড | ভাষা | README |
|-----|------|--------|-----|------|--------|-----|------|--------|
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

## কিবোর্ড শর্টকাট

| কী | কার্যক্রম |
|----|-----------|
| `q`, `Esc` | প্রস্থান করুন |
| `h`, `?` | সহায়তা চালু/বন্ধ করুন |
| `↑`, `k` | উপরে যান |
| `↓`, `j` | নিচে যান |
| `g`, `Home` | শীর্ষে যান |
| `G`, `End` | সর্বনিম্নে যান |
| `PgUp` | এক পৃষ্ঠা উপরে |
| `PgDn` | এক পৃষ্ঠা নিচে |
| `s` | সাজানোর ক্ষেত্র পরিবর্তন করুন |
| `r` | সাজানোর ক্রম বিপরীত করুন |
| `1-7` | কলাম অনুসারে সাজান |
| `Enter` | নির্বাচিত সিম্বলের বিস্তারিত পপআপ প্রদর্শন করুন |
| `/` | সিম্বল অনুসন্ধান/ফিল্টার করুন |
| `a` | ওয়াচলিস্টে একটি সিম্বল যোগ করুন |
| `d` | নির্বাচিত সিম্বল অপসারণ করুন |
| `S` | স্পার্কলাইন চার্ট সাইডবার চালু/বন্ধ করুন |
| `e` | জেসন ফরম্যাটে ওয়াচলিস্ট এক্সপোর্ট করুন |
| `H` | হোল্ডিংস ভিউ চালু/বন্ধ করুন |
| `f` | মৌলিক আর্থিক তথ্য চালু/বন্ধ করুন |
| `Space`, `R` | জোরপূর্বক রিফ্রেশ করুন |
| `Tab` | সিম্বল গ্রুপসমূহের মধ্যে পরিবর্তন করুন |

## কনফিগারেশন

একটি টিওএমএল কনফিগারেশন ফাইল তৈরি করুন:

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

# পোর্টফোলিও হোল্ডিংস (ঐচ্ছিক)
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

# সিম্বল গ্রুপসমূহ
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# বহু-মুদ্রা প্রদর্শন (ঐচ্ছিক)
[currency]
display = "EUR"
convert = true

# মূল্য সতর্কতা (ঐচ্ছিক)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# কাস্টম ক্রিপ্টো শর্টকাট (ঐচ্ছিক)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## সিম্বল ফরম্যাট

| ফরম্যাট | বিবরণ | উদাহরণ |
|----------|-------|--------|
| শেয়ার | প্রচলিত টিকার | `AAPL`, `GOOGL` |
| ক্রিপ্টো | ইউএসডি প্রত্যয়সহ | `BTC-USD`, `ETH-USD` |
| ক্রিপ্টো সংক্ষিপ্ত রূপ | স্বয়ংক্রিয়ভাবে `-USD` যুক্ত হয় | `BTC.X` -> `BTC-USD` |
| স্বয়ংক্রিয় ক্রিপ্টো | জনপ্রিয় ক্রিপ্টোকারেন্সি | `BTC` -> `BTC-USD` |

## তথ্যের উৎস

রিয়েল-টাইম বাজারদর সংগ্রহের জন্য স্টঙ্কটপ ইয়াহু ফাইন্যান্স এপিআই ব্যবহার করে। কোনো এপিআই কী-এর প্রয়োজন নেই।

## উন্নয়ন

### বিল্ড

```bash
# ডিবাগ বিল্ড
cargo build

# রিলিজ বিল্ড
cargo build --release
```

### পরীক্ষা

```bash
# সকল পরীক্ষা চালান
cargo test

# আউটপুটসহ পরীক্ষা চালান
cargo test -- --nocapture
```

### লিন্ট

```bash
# লিন্টিং সমস্যা পরীক্ষা করুন
cargo clippy --all-targets --all-features

# স্বয়ংক্রিয় সংশোধন প্রয়োগ করুন
cargo clippy --fix --allow-dirty
```

### ফরম্যাট

```bash
# ফরম্যাটিং পরীক্ষা করুন
cargo fmt -- --check

# ফরম্যাটিং প্রয়োগ করুন
cargo fmt
```

## অবদান

আপনার অবদান সাদরে গৃহীত! নির্দ্বিধায় একটি পুল রিকোয়েস্ট জমা দিন।

1. রিপোজিটরিটি ফর্ক করুন
2. আপনার ফিচার শাখা তৈরি করুন (`git checkout -b feature/amazing-feature`)
3. আপনার পরিবর্তনগুলো কমিট করুন (`git commit -m 'Add amazing feature'`)
4. শাখাটি পুশ করুন (`git push origin feature/amazing-feature`)
5. একটি পুল রিকোয়েস্ট খুলুন

## লাইসেন্স

এই প্রকল্পটি এমআইটি লাইসেন্সের অধীনে লাইসেন্সপ্রাপ্ত — বিস্তারিত জানতে [LICENSE](../../../LICENSE) ফাইলটি দেখুন।

## কৃতজ্ঞতা

- অনুপ্রাণিত: [ticker](https://github.com/achannarasappa/ticker)
- নির্মিত: [ratatui](https://github.com/ratatui-org/ratatui)
- তথ্যসূত্র: [Yahoo Finance](https://finance.yahoo.com/)
