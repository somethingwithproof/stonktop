# Stonktop

> [English](../../../README.md)

Interfejs terminalowy wzorowany na `top` do monitorowania notowań akcji i kryptowalut w czasie rzeczywistym.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Funkcje

- **Monitorowanie cen w czasie rzeczywistym**: Bieżące notowania akcji i kryptowalut
- **Interfejs w stylu `top`**: Znajoma obsługa klawiaturą podobna do `top`/`htop`
- **Wykresy sparkline**: Wbudowana wizualizacja trendów cenowych w panelu bocznym
- **Śledzenie portfela**: Monitorowanie posiadanych pozycji wraz z obliczaniem zysków i strat (P/L)
- **Wyszukiwanie i filtrowanie**: Wyszukiwanie symboli i nazw bez rozróżniania wielkości liter
- **Alerty cenowe**: Powiadomienia systemowe po przekroczeniu skonfigurowanych progów cenowych
- **Eksport/Import**: Zapisywanie i wczytywanie list obserwowanych w formacie JSON lub CSV
- **Obsługa wielu walut**: Wyświetlanie cen w dowolnej walucie z bieżącymi kursami wymiany
- **Wiele opcji sortowania**: Sortowanie według symbolu, ceny, zmiany, wolumenu lub kapitalizacji rynkowej
- **Tryb wsadowy (batch)**: Nieinteraktywne wyjście przeznaczone do skryptów (jak `top -b`)
- **Konfiguracja TOML**: Łatwe do edycji pliki konfiguracyjne
- **Kodowanie kolorami**: Zielony dla wzrostów, czerwony dla spadków
- **Skróty dla kryptowalut**: Możliwość użycia `BTC.X` lub po prostu `BTC` zamiast `BTC-USD`

## Zrzuty ekranu

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

## Wymagania wstępne

- **Rust**: Wersja 1.88 lub nowsza (MSRV)
- **Cargo**: Dołączone do instalacji języka Rust

## Instalacja

### Z crates.io (Zalecane)

```bash
cargo install stonktop
```

### Za pomocą cargo-binstall

Jeśli masz zainstalowane narzędzie [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), możesz zainstalować gotowe pliki binarne:

```bash
cargo binstall stonktop
```

### Gotowe pliki binarne

Pobierz gotowe pliki binarne ze strony [Releases](https://github.com/somethingwithproof/stonktop/releases).

Dostępne platformy:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Ze źródeł

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Użycie

### Podstawowe użycie

```bash
# Monitorowanie wybranych symboli
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Ze skrótami dla kryptowalut
stonktop -s AAPL,BTC,ETH,SOL
```

### Opcje w stylu `top`

```bash
# Ustawienie interwału odświeżania (jak top -d)
stonktop -s AAPL,GOOGL -d 10

# Wykonanie N iteracji i zakończenie działania (jak top -n)
stonktop -s AAPL -n 5

# Tryb wsadowy do skryptów (jak top -b)
stonktop -s AAPL,GOOGL -b

# Tryb bezpieczny — wyłączenie poleceń interaktywnych
stonktop -s AAPL -S
```

### Opcje sortowania

```bash
# Sortowanie według ceny (malejąco)
stonktop -s AAPL,GOOGL -o price

# Sortowanie według symbolu (rosnąco)
stonktop -s AAPL,GOOGL -o symbol -r

# Dostępne pola sortowania: symbol, name, price, change, change-percent, volume, market-cap
```

### Plik konfiguracyjny

```bash
# Użycie konkretnego pliku konfiguracyjnego
stonktop -c ~/.config/stonktop/custom.toml

# Lub umieszczenie konfiguracji w domyślnej lokalizacji:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opcje wiersza poleceń

| Opcja | Skrót | Opis |
|-------|-------|------|
| `--symbols` | `-s` | Lista obserwowanych symboli rozdzielona przecinkami |
| `--delay` | `-d` | Interwał odświeżania w sekundach (domyślnie: 5) |
| `--iterations` | `-n` | Liczba iteracji (0 = nieskończona) |
| `--batch` | `-b` | Tryb wsadowy — wyjście nieinteraktywne |
| `--secure` | `-S` | Tryb bezpieczny — wyłączenie poleceń interaktywnych |
| `--config` | `-c` | Ścieżka do pliku konfiguracyjnego |
| `--sort` | `-o` | Początkowe pole sortowania |
| `--reverse` | `-r` | Odwrócenie kolejności sortowania |
| `--holdings` | `-H` | Wyświetlenie widoku pozycji/portfela |
| `--currency` | | Waluta wyświetlania (domyślnie: USD) |
| `--timeout` | | Limit czasu odpowiedzi API w sekundach (domyślnie: 10) |
| `--format` | | Format wyjścia dla trybu wsadowego: `table`, `json`, `csv` |
| `--color` | | Kolorowe wyjście: `auto`, `always`, `never` |
| `--verbose` | `-v` | Szczegółowe wyjście |
| `--init` | | Wygenerowanie przykładowego pliku konfiguracyjnego |
| `--force` | | Wymuszenie nadpisania istniejącej konfiguracji (z `--init`) |
| `--lang` | | Język wyświetlania (`STONKTOP_LANG`, domyślnie: automatycznie z `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Wyświetlenie komunikatu pomocy |
| `--version` | `-V` | Wyświetlenie wersji |

## Obsługiwane języki

Stonktop wykrywa język wyświetlania na podstawie zmiennych `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` i `LANG` lub przyjmuje opcję `--lang <LANG>`. Wszystkie 45 obsługiwanych ustawień regionalnych zawiera pełne tłumaczenia CLI i TUI (w tym wyrównanie od prawej do lewej dla języków arabskiego, perskiego, hebrajskiego i urdu) oraz zlokalizowaną dokumentację:

| Kod | Język | README | Kod | Język | README | Kod | Język | README |
|-----|-------|--------|-----|-------|--------|-----|-------|--------|
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

## Skróty klawiszowe

| Klawisz | Akcja |
|---------|-------|
| `q`, `Esc` | Wyjście |
| `h`, `?` | Przełączenie pomocy |
| `↑`, `k` | Przejście w górę |
| `↓`, `j` | Przejście w dół |
| `g`, `Home` | Przejście na początek |
| `G`, `End` | Przejście na koniec |
| `PgUp` | Strona w górę |
| `PgDn` | Strona w dół |
| `s` | Zmiana pola sortowania |
| `r` | Odwrócenie kolejności sortowania |
| `1-7` | Sortowanie według kolumny |
| `Enter`, `d` | Wyświetlenie okna szczegółów dla wybranego symbolu |
| `/` | Wyszukiwanie/filtrowanie symboli |
| `a` | Dodanie symbolu do listy obserwowanych |
| `x` | Usunięcie wybranego symbolu |
| `S` | Przełączenie panelu bocznego z wykresami sparkline |
| `e` | Eksport listy obserwowanych do JSON |
| `H` | Przełączenie widoku pozycji |
| `f` | Przełączenie danych fundamentalnych |
| `Space`, `R` | Wymuszenie odświeżenia |
| `Tab` | Przełączanie grup symboli |

## Konfiguracja

Utwórz plik konfiguracyjny w formacie TOML:

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

# Pozycje portfela (opcjonalnie)
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

# Grupy symboli
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Wyświetlanie w wielu walutach (opcjonalnie)
[currency]
display = "EUR"
convert = true

# Alerty cenowe (opcjonalnie)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Niestandardowe skróty dla kryptowalut (opcjonalnie)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formaty symboli

| Format | Opis | Przykład |
|--------|------|----------|
| Akcja | Standardowy symbol giełdowy | `AAPL`, `GOOGL` |
| Kryptowaluta | Z przyrostkiem USD | `BTC-USD`, `ETH-USD` |
| Skrót krypto | Rozwijany do `-USD` | `BTC.X` -> `BTC-USD` |
| Automatyczne krypto | Popularne kryptowaluty | `BTC` -> `BTC-USD` |

## Źródło danych

Stonktop korzysta z API Yahoo Finance do pobierania notowań w czasie rzeczywistym. Klucz API nie jest wymagany.

## Rozwój

### Budowanie

```bash
# Kompilacja debug
cargo build

# Kompilacja release
cargo build --release
```

### Testy

```bash
# Uruchomienie wszystkich testów
cargo test

# Uruchomienie testów z wyświetlaniem wyjścia
cargo test -- --nocapture
```

### Analiza statyczna (Lint)

```bash
# Sprawdzenie problemów lintera
cargo clippy --all-targets --all-features

# Zastosowanie automatycznych poprawek
cargo clippy --fix --allow-dirty
```

### Formatowanie

```bash
# Sprawdzenie formatowania
cargo fmt -- --check

# Zastosowanie formatowania
cargo fmt
```

## Współtworzenie

Wszelki wkład jest mile widziany! Zachęcamy do przesyłania Pull Requestów.

1. Utwórz fork repozytorium
2. Utwórz gałąź z nową funkcją (`git checkout -b feature/amazing-feature`)
3. Zatwierdź swoje zmiany (`git commit -m 'Add amazing feature'`)
4. Wypchnij gałąź (`git push origin feature/amazing-feature`)
5. Otwórz Pull Request

## Licencja

Ten projekt jest udostępniany na licencji MIT — szczegóły znajdują się w pliku [LICENSE](../../../LICENSE).

## Podziękowania

- Zainspirowane projektem [ticker](https://github.com/achannarasappa/ticker)
- Zbudowane z użyciem [ratatui](https://github.com/ratatui-org/ratatui)
- Dane pochodzą z [Yahoo Finance](https://finance.yahoo.com/)
