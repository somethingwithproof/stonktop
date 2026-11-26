# Stonktop

> [English](../../../README.md)

Hisse senedi ve kripto para fiyatlarını gerçek zamanlı olarak izlemek için `top` benzeri bir terminal kullanıcı arayüzü.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Özellikler

- **Gerçek Zamanlı Fiyat İzleme**: Hisse senetleri ve kripto paralar için canlı fiyat verileri
- **`top` Benzeri Arayüz**: `top`/`htop` ile benzer, alışıldık klavye kontrolleri
- **Sparkline Grafikleri**: Yan panelde satır içi fiyat trendi görselleştirmesi
- **Portföy Takibi**: Kâr/zarar (P/L) hesaplamalarıyla varlıklarınızı izleyin
- **Arama ve Filtreleme**: Semboller ve adlar arasında büyük/küçük harf duyarsız arama
- **Fiyat Alarmları**: Fiyatlar yapılandırılan eşikleri geçtiğinde masaüstü bildirimleri
- **Dışa/İçe Aktarma**: İzleme listelerini JSON veya CSV olarak kaydedin ve yükleyin
- **Çoklu Para Birimi**: Canlı döviz kurlarıyla fiyatları herhangi bir para biriminde görüntüleyin
- **Çoklu Sıralama Seçenekleri**: Sembol, fiyat, değişim, hacim ve piyasa değerine göre sıralama
- **Toplu İş (Batch) Modu**: Betik yazımı için etkileşimsiz çıktı (`top -b` gibi)
- **TOML Yapılandırması**: Kolayca düzenlenebilir yapılandırma dosyaları
- **Renk Kodlaması**: Yükselişler için yeşil, düşüşler için kırmızı
- **Kripto Kısayolları**: `BTC-USD` yerine `BTC.X` veya yalnızca `BTC` kullanın

## Ekran Görüntüleri

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

## Ön Koşullar

- **Rust**: 1.88 veya üzeri sürüm (MSRV)
- **Cargo**: Rust kurulumuyla birlikte gelir

## Kurulum

### crates.io Üzerinden (Önerilen)

```bash
cargo install stonktop
```

### cargo-binstall Kullanarak

Sisteminizde [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) kuruluysa önceden derlenmiş ikili dosyaları kurabilirsiniz:

```bash
cargo binstall stonktop
```

### Önceden Derlenmiş İkili Dosyalar

Önceden derlenmiş ikili dosyaları [Releases](https://github.com/somethingwithproof/stonktop/releases) sayfasından indirebilirsiniz.

Desteklenen platformlar:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Kaynak Koddan Derleme

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Kullanım

### Temel Kullanım

```bash
# Belirli sembolleri izle
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Kripto kısayollarıyla
stonktop -s AAPL,BTC,ETH,SOL
```

### `top` Benzeri Seçenekler

```bash
# Yenileme gecikmesini ayarla (top -d gibi)
stonktop -s AAPL,GOOGL -d 10

# N yineleme çalıştır ve çık (top -n gibi)
stonktop -s AAPL -n 5

# Betikler için toplu iş modu (top -b gibi)
stonktop -s AAPL,GOOGL -b

# Güvenli mod - etkileşimli komutları devre dışı bırak
stonktop -s AAPL -S
```

### Sıralama Seçenekleri

```bash
# Fiyata göre sırala (azalan)
stonktop -s AAPL,GOOGL -o price

# Sembole göre sırala (artan)
stonktop -s AAPL,GOOGL -o symbol -r

# Kullanılabilir sıralama alanları: symbol, name, price, change, change-percent, volume, market-cap
```

### Yapılandırma Dosyası

```bash
# Belirli bir yapılandırma dosyasını kullan
stonktop -c ~/.config/stonktop/custom.toml

# Veya yapılandırmayı varsayılan konuma yerleştirin:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Komut Satırı Seçenekleri

| Seçenek | Kısa | Açıklama |
|---------|------|----------|
| `--symbols` | `-s` | İzlenecek sembollerin virgülle ayrılmış listesi |
| `--delay` | `-d` | Saniye cinsinden yenileme gecikmesi (varsayılan: 5) |
| `--iterations` | `-n` | Yineleme sayısı (0 = sınırsız) |
| `--batch` | `-b` | Toplu iş modu - etkileşimsiz çıktı |
| `--secure` | `-S` | Güvenli mod - etkileşimli komutları devre dışı bırakır |
| `--config` | `-c` | Yapılandırma dosyasının yolu |
| `--sort` | `-o` | Başlangıç sıralama alanı |
| `--reverse` | `-r` | Sıralama düzenini tersine çevir |
| `--holdings` | `-H` | Varlık/portföy görünümünü göster |
| `--currency` | | Görüntüleme para birimi (varsayılan: USD) |
| `--timeout` | | Saniye cinsinden API zaman aşımı (varsayılan: 10) |
| `--format` | | Toplu iş modu çıktı biçimi: table, json, csv |
| `--color` | | Renkli çıktı: auto, always, never |
| `--verbose` | `-v` | Ayrıntılı çıktı |
| `--init` | | Örnek bir yapılandırma dosyası oluştur |
| `--force` | | Mevcut yapılandırmanın üzerine zorla yaz (`--init` ile) |
| `--lang` | | Görüntüleme dili (`STONKTOP_LANG`, varsayılan: `LC_ALL`/`LC_MESSAGES`/`LANG` üzerinden otomatik) |
| `--help` | | Yardım iletisini göster |
| `--version` | `-V` | Sürümü göster |

## Desteklenen Diller

Stonktop, görüntüleme dilini `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` ve `LANG` değişkenlerinden otomatik olarak algılar veya `--lang <LANG>` parametresini kabul eder. Desteklenen 45 yerelin tamamı eksiksiz CLI ve TUI çevirilerini (Arapça, Farsça, İbranice ve Urduca için sağdan sola hizalama dâhil) ve yerelleştirilmiş belgeleri içerir:

| Kod | Dil | README | Kod | Dil | README | Kod | Dil | README |
|-----|-----|--------|-----|-----|--------|-----|-----|--------|
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

## Klavye Kısayolları

| Tuş | İşlem |
|-----|-------|
| `q`, `Esc` | Çık |
| `h`, `?` | Yardımı aç/kapat |
| `↑`, `k` | Yukarı git |
| `↓`, `j` | Aşağı git |
| `g`, `Home` | En üste git |
| `G`, `End` | En alta git |
| `PgUp` | Bir sayfa yukarı |
| `PgDn` | Bir sayfa aşağı |
| `s` | Sıralama alanını değiştir |
| `r` | Sıralama düzenini tersine çevir |
| `1-7` | Sütuna göre sırala |
| `Enter`, `d` | Seçili sembol için ayrıntı penceresini göster |
| `/` | Sembolleri ara/filtrele |
| `a` | İzleme listesine sembol ekle |
| `x` | Seçili sembolü kaldır |
| `S` | Sparkline grafikleri yan panelini aç/kapat |
| `e` | İzleme listesini JSON olarak dışa aktar |
| `H` | Varlık görünümünü aç/kapat |
| `f` | Temel göstergeleri aç/kapat |
| `Space`, `R` | Yenilemeye zorla |
| `Tab` | Sembol grupları arasında geçiş yap |

## Yapılandırma

Bir TOML yapılandırma dosyası oluşturun:

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

# Portföy varlıkları (isteğe bağlı)
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

# Sembol grupları
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Çoklu para birimi görünümü (isteğe bağlı)
[currency]
display = "EUR"
convert = true

# Fiyat alarmları (isteğe bağlı)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Özel kripto kısayolları (isteğe bağlı)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Sembol Biçimleri

| Biçim | Açıklama | Örnek |
|-------|----------|-------|
| Hisse senedi | Standart borsa kodu | `AAPL`, `GOOGL` |
| Kripto | USD son ekiyle | `BTC-USD`, `ETH-USD` |
| Kripto kısa gösterim | `-USD` olarak genişletilir | `BTC.X` -> `BTC-USD` |
| Otomatik kripto | Yaygın kripto paralar | `BTC` -> `BTC-USD` |

## Veri Kaynağı

Stonktop, gerçek zamanlı fiyat verilerini almak için Yahoo Finance API'sini kullanır. API anahtarı gerekmez.

## Geliştirme

### Derleme

```bash
# Hata ayıklama (debug) derlemesi
cargo build

# Sürüm (release) derlemesi
cargo build --release
```

### Test

```bash
# Tüm testleri çalıştır
cargo test

# Testleri çıktılarıyla birlikte çalıştır
cargo test -- --nocapture
```

### Statik Analiz (Lint)

```bash
# Lint sorunlarını denetle
cargo clippy --all-targets --all-features

# Otomatik düzeltmeleri uygula
cargo clippy --fix --allow-dirty
```

### Biçimlendirme

```bash
# Biçimlendirmeyi denetle
cargo fmt -- --check

# Biçimlendirmeyi uygula
cargo fmt
```

## Katkıda Bulunma

Katkılarınızı memnuniyetle karşılıyoruz! Pull Request göndermekten çekinmeyin.

1. Depoyu çatallayın (fork)
2. Özellik dalınızı oluşturun (`git checkout -b feature/amazing-feature`)
3. Değişikliklerinizi işleyin (`git commit -m 'Add amazing feature'`)
4. Dalı uzak depoya gönderin (`git push origin feature/amazing-feature`)
5. Bir Pull Request açın

## Lisans

Bu proje MIT Lisansı ile lisanslanmıştır; ayrıntılar için [LICENSE](../../../LICENSE) dosyasına bakın.

## Teşekkürler

- [ticker](https://github.com/achannarasappa/ticker) projesinden esinlenilmiştir
- [ratatui](https://github.com/ratatui-org/ratatui) ile geliştirilmiştir
- Veriler [Yahoo Finance](https://finance.yahoo.com/) üzerinden sağlanmaktadır
