# Stonktop

> [English](../../../README.md)

Eine an `top` angelehnte Terminal-Benutzeroberfläche zur Überwachung von Aktien- und Kryptowährungskursen in Echtzeit.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Funktionen

- **Kursüberwachung in Echtzeit**: Aktuelle Kurse für Aktien und Kryptowährungen
- **`top`-ähnliche Oberfläche**: Vertraute Tastatursteuerung wie bei `top`/`htop`
- **Sparkline-Diagramme**: Eingebettete Kursverlaufs-Visualisierung in einer Seitenleiste
- **Portfolio-Verfolgung**: Überwachung eigener Bestände inklusive Gewinn-/Verlustberechnung (GuV)
- **Suche & Filter**: Groß-/Kleinschreibung-unabhängige Suche über Symbole und Namen
- **Kursalarme**: Desktop-Benachrichtigungen beim Über- oder Unterschreiten konfigurierter Schwellenwerte
- **Export/Import**: Speichern und Laden von Beobachtungslisten als JSON oder CSV
- **Mehrwährungsunterstützung**: Anzeige von Kursen in beliebigen Währungen mit aktuellen Wechselkursen
- **Vielfältige Sortieroptionen**: Sortierung nach Symbol, Kurs, Änderung, Volumen und Marktkapitalisierung
- **Batch-Modus**: Nicht-interaktive Ausgabe für Skripte (analog zu `top -b`)
- **TOML-Konfiguration**: Einfach zu bearbeitende Konfigurationsdateien
- **Farbcodierung**: Grün für Gewinne, Rot für Verluste
- **Krypto-Kürzel**: `BTC.X` oder einfach `BTC` für `BTC-USD` verwenden

## Bildschirmfotos

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

## Voraussetzungen

- **Rust**: Version 1.89 oder höher (MSRV)
- **Cargo**: In der Rust-Installation enthalten

## Installation

### Über crates.io (Empfohlen)

```bash
cargo install stonktop
```

### Mit cargo-binstall

Wenn [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installiert ist, können vorkompilierte Binärdateien direkt bezogen werden:

```bash
cargo binstall stonktop
```

### Vorkompilierte Binärdateien

Vorkompilierte Binärdateien stehen auf der Seite [Releases](https://github.com/somethingwithproof/stonktop/releases) zum Download bereit.

Verfügbare Plattformen:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Aus dem Quellcode

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Verwendung

### Grundlegende Verwendung

```bash
# Bestimmte Symbole beobachten
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Mit Krypto-Kürzeln
stonktop -s AAPL,BTC,ETH,SOL
```

### `top`-ähnliche Optionen

```bash
# Aktualisierungsintervall festlegen (wie top -d)
stonktop -s AAPL,GOOGL -d 10

# N Durchläufe ausführen und anschließend beenden (wie top -n)
stonktop -s AAPL -n 5

# Batch-Modus für Skripte (wie top -b)
stonktop -s AAPL,GOOGL -b

# Sicherheitsmodus – interaktive Befehle deaktivieren
stonktop -s AAPL -S
```

### Sortieroptionen

```bash
# Nach Kurs sortieren (absteigend)
stonktop -s AAPL,GOOGL -o price

# Nach Symbol sortieren (aufsteigend)
stonktop -s AAPL,GOOGL -o symbol -r

# Verfügbare Sortierfelder: symbol, name, price, change, change-percent, volume, market-cap
```

### Konfigurationsdatei

```bash
# Eine bestimmte Konfigurationsdatei verwenden
stonktop -c ~/.config/stonktop/custom.toml

# Oder die Konfiguration am Standardpfad ablegen:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Kommandozeilenoptionen

| Option | Kurzform | Beschreibung |
|--------|----------|--------------|
| `--symbols` | `-s` | Kommagetrennte Liste der zu beobachtenden Symbole |
| `--delay` | `-d` | Aktualisierungsintervall in Sekunden (Standard: 5) |
| `--iterations` | `-n` | Anzahl der Durchläufe (0 = unbegrenzt) |
| `--batch` | `-b` | Batch-Modus – nicht-interaktive Ausgabe |
| `--secure` | `-S` | Sicherheitsmodus – interaktive Befehle deaktivieren |
| `--config` | `-c` | Pfad zur Konfigurationsdatei |
| `--sort` | `-o` | Anfängliches Sortierfeld |
| `--reverse` | `-r` | Sortierreihenfolge umkehren |
| `--holdings` | `-H` | Bestands-/Portfolioansicht anzeigen |
| `--currency` | | Anzeigewährung (Standard: USD) |
| `--timeout` | | API-Zeitüberschreitung in Sekunden (Standard: 10) |
| `--format` | | Ausgabeformat im Batch-Modus: `table`, `json`, `csv` |
| `--color` | | Farbausgabe: `auto`, `always`, `never` |
| `--verbose` | `-v` | Ausführliche Ausgabe |
| `--init` | | Eine Beispiel-Konfigurationsdatei erzeugen |
| `--force` | | Vorhandene Konfiguration überschreiben (mit `--init`) |
| `--lang` | | Anzeigesprache (`STONKTOP_LANG`, Standard: automatisch aus `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Hilfemeldung anzeigen |
| `--version` | `-V` | Versionsnummer anzeigen |

## Unterstützte Sprachen

Stonktop erkennt die Anzeigesprache anhand von `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` und `LANG` oder über die Option `--lang <LANG>`. Alle 48 unterstützten Sprachregionen umfassen vollständige CLI- und TUI-Übersetzungen (einschließlich Rechts-nach-links-Ausrichtung für Arabisch, Persisch, Hebräisch und Urdu) sowie lokalisierte Dokumentation:

| Code | Sprache | README | Code | Sprache | README | Code | Sprache | README |
|------|---------|--------|------|---------|--------|------|---------|--------|
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

## Tastenkürzel

| Taste | Aktion |
|-------|--------|
| `q`, `Esc` | Beenden |
| `h`, `?` | Hilfe ein-/ausblenden |
| `↑`, `k` | Nach oben bewegen |
| `↓`, `j` | Nach unten bewegen |
| `g`, `Home` | Zum Anfang springen |
| `G`, `End` | Zum Ende springen |
| `PgUp` | Eine Seite nach oben |
| `PgDn` | Eine Seite nach unten |
| `s` | Sortierfeld wechseln |
| `r` | Sortierreihenfolge umkehren |
| `1-7` | Nach Spalte sortieren |
| `Enter` | Detailansicht für das ausgewählte Symbol öffnen |
| `/` | Symbole suchen/filtern |
| `a` | Symbol zur Beobachtungsliste hinzufügen |
| `d` | Ausgewähltes Symbol entfernen |
| `S` | Seitenleiste mit Sparkline-Diagrammen ein-/ausblenden |
| `e` | Beobachtungsliste als JSON exportieren |
| `H` | Bestandsansicht ein-/ausblenden |
| `f` | Fundamentaldaten ein-/ausblenden |
| `Space`, `R` | Aktualisierung erzwingen |
| `Tab` | Symbolgruppen durchschalten |

## Konfiguration

Erstellen Sie eine TOML-Konfigurationsdatei:

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

# Portfoliobestände (optional)
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

# Symbolgruppen
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Mehrwährungsanzeige (optional)
[currency]
display = "EUR"
convert = true

# Kursalarme (optional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Benutzerdefinierte Krypto-Kürzel (optional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Symbolformate

| Format | Beschreibung | Beispiel |
|--------|--------------|----------|
| Aktie | Standard-Tickersymbol | `AAPL`, `GOOGL` |
| Krypto | Mit USD-Suffix | `BTC-USD`, `ETH-USD` |
| Krypto-Kurzform | Wird zu `-USD` erweitert | `BTC.X` -> `BTC-USD` |
| Krypto-Automatik | Gängige Kryptowährungen | `BTC` -> `BTC-USD` |

## Datenquelle

Stonktop nutzt die Yahoo-Finance-API zum Abruf von Echtzeitkursen. Ein API-Schlüssel ist nicht erforderlich.

## Entwicklung

### Erstellen

```bash
# Debug-Build
cargo build

# Release-Build
cargo build --release
```

### Testen

```bash
# Alle Tests ausführen
cargo test

# Tests mit Standardausgabe ausführen
cargo test -- --nocapture
```

### Linting

```bash
# Auf Linting-Probleme prüfen
cargo clippy --all-targets --all-features

# Automatische Korrekturen anwenden
cargo clippy --fix --allow-dirty
```

### Formatierung

```bash
# Formatierung prüfen
cargo fmt -- --check

# Formatierung anwenden
cargo fmt
```

## Beitragen

Beiträge sind herzlich willkommen! Reichen Sie gerne einen Pull Request ein.

1. Repository forken
2. Feature-Branch erstellen (`git checkout -b feature/amazing-feature`)
3. Änderungen committen (`git commit -m 'Add amazing feature'`)
4. Branch pushen (`git push origin feature/amazing-feature`)
5. Pull Request öffnen

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz – weitere Einzelheiten finden Sie in der Datei [LICENSE](../../../LICENSE).

## Danksagungen

- Inspiriert von [ticker](https://github.com/achannarasappa/ticker)
- Entwickelt mit [ratatui](https://github.com/ratatui-org/ratatui)
- Kursdaten von [Yahoo Finance](https://finance.yahoo.com/)
