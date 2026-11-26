# Stonktop

> [English](../../../README.md)

Un'interfaccia da terminale in stile `top` per monitorare le quotazioni di azioni e criptovalute in tempo reale.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Funzionalità

- **Monitoraggio dei prezzi in tempo reale**: Quotazioni in tempo reale per azioni e criptovalute
- **Interfaccia in stile `top`**: Comandi da tastiera familiari simili a `top`/`htop`
- **Grafici sparkline**: Visualizzazione integrata dell'andamento dei prezzi in una barra laterale
- **Monitoraggio del portafoglio**: Tieni sotto controllo le tue posizioni con calcolo di profitti e perdite (P/L)
- **Ricerca e filtri**: Ricerca senza distinzione tra maiuscole e minuscole su simboli e nomi
- **Avvisi di prezzo**: Notifiche desktop quando i prezzi superano le soglie configurate
- **Esportazione/Importazione**: Salva e carica liste di controllo in formato JSON o CSV
- **Multivaluta**: Visualizza i prezzi in qualsiasi valuta con tassi di cambio aggiornati in tempo reale
- **Opzioni di ordinamento multiple**: Ordina per simbolo, prezzo, variazione, volume o capitalizzazione di mercato
- **Modalità batch**: Output non interattivo per l'utilizzo in script (come `top -b`)
- **Configurazione TOML**: File di configurazione semplici da modificare
- **Codifica a colori**: Verde per i rialzi, rosso per i ribassi
- **Scorciatoie cripto**: Usa `BTC.X` o semplicemente `BTC` per `BTC-USD`

## Schermate

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

## Prerequisiti

- **Rust**: Versione 1.89 o superiore (MSRV)
- **Cargo**: Incluso nell'installazione di Rust

## Installazione

### Da crates.io (Consigliato)

```bash
cargo install stonktop
```

### Tramite cargo-binstall

Se [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) è installato, è possibile installare i binari precompilati:

```bash
cargo binstall stonktop
```

### Binari precompilati

Scarica i binari precompilati dalla pagina [Releases](https://github.com/somethingwithproof/stonktop/releases).

Piattaforme disponibili:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Dal codice sorgente

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Utilizzo

### Utilizzo di base

```bash
# Monitorare simboli specifici
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Con scorciatoie cripto
stonktop -s AAPL,BTC,ETH,SOL
```

### Opzioni in stile `top`

```bash
# Impostare l'intervallo di aggiornamento (come top -d)
stonktop -s AAPL,GOOGL -d 10

# Eseguire N iterazioni e uscire (come top -n)
stonktop -s AAPL -n 5

# Modalità batch per script (come top -b)
stonktop -s AAPL,GOOGL -b

# Modalità sicura: disabilitare i comandi interattivi
stonktop -s AAPL -S
```

### Opzioni di ordinamento

```bash
# Ordinare per prezzo (decrescente)
stonktop -s AAPL,GOOGL -o price

# Ordinare per simbolo (crescente)
stonktop -s AAPL,GOOGL -o symbol -r

# Campi di ordinamento disponibili: symbol, name, price, change, change-percent, volume, market-cap
```

### File di configurazione

```bash
# Utilizzare un file di configurazione specifico
stonktop -c ~/.config/stonktop/custom.toml

# Oppure posizionare la configurazione nel percorso predefinito:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opzioni da riga di comando

| Opzione | Forma breve | Descrizione |
|---------|-------------|-------------|
| `--symbols` | `-s` | Elenco di simboli da monitorare separati da virgole |
| `--delay` | `-d` | Intervallo di aggiornamento in secondi (predefinito: 5) |
| `--iterations` | `-n` | Numero di iterazioni (0 = infinito) |
| `--batch` | `-b` | Modalità batch — output non interattivo |
| `--secure` | `-S` | Modalità sicura — disabilita i comandi interattivi |
| `--config` | `-c` | Percorso del file di configurazione |
| `--sort` | `-o` | Campo di ordinamento iniziale |
| `--reverse` | `-r` | Inverti l'ordine di ordinamento |
| `--holdings` | `-H` | Mostra la vista delle posizioni/portafoglio |
| `--currency` | | Valuta di visualizzazione (predefinita: USD) |
| `--timeout` | | Timeout dell'API in secondi (predefinito: 10) |
| `--format` | | Formato di output per la modalità batch: `table`, `json`, `csv` |
| `--color` | | Output a colori: `auto`, `always`, `never` |
| `--verbose` | `-v` | Output dettagliato |
| `--init` | | Genera un file di configurazione di esempio |
| `--force` | | Forza la sovrascrittura della configurazione esistente (con `--init`) |
| `--lang` | | Lingua di visualizzazione (`STONKTOP_LANG`, predefinita: automatica da `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Mostra il messaggio di aiuto |
| `--version` | `-V` | Mostra la versione |

## Lingue supportate

Stonktop rileva la lingua di visualizzazione da `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` e `LANG`, oppure accetta `--lang <LANG>`. Tutte le 45 impostazioni locali supportate includono traduzioni complete della CLI e della TUI (compreso l'allineamento da destra a sinistra per arabo, persiano, ebraico e urdu) e documentazione localizzata:

| Codice | Lingua | README | Codice | Lingua | README | Codice | Lingua | README |
|--------|--------|--------|--------|--------|--------|--------|--------|--------|
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

## Scorciatoie da tastiera

| Tasto | Azione |
|-------|--------|
| `q`, `Esc` | Esci |
| `h`, `?` | Mostra/nascondi la guida |
| `↑`, `k` | Sposta in alto |
| `↓`, `j` | Sposta in basso |
| `g`, `Home` | Vai all'inizio |
| `G`, `End` | Vai alla fine |
| `PgUp` | Pagina precedente |
| `PgDn` | Pagina successiva |
| `s` | Scorri i campi di ordinamento |
| `r` | Inverti l'ordine di ordinamento |
| `1-7` | Ordina per colonna |
| `Enter` | Mostra finestra di dettaglio per il simbolo selezionato |
| `/` | Cerca/filtra simboli |
| `a` | Aggiungi un simbolo alla lista di controllo |
| `d` | Rimuovi il simbolo selezionato |
| `S` | Mostra/nascondi la barra laterale con i grafici sparkline |
| `e` | Esporta la lista di controllo in JSON |
| `H` | Mostra/nascondi la vista posizioni |
| `f` | Mostra/nascondi i dati fondamentali |
| `Space`, `R` | Forza l'aggiornamento |
| `Tab` | Scorri i gruppi di simboli |

## Configurazione

Crea un file di configurazione TOML:

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

# Posizioni di portafoglio (facoltativo)
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

# Gruppi di simboli
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Visualizzazione multivaluta (facoltativo)
[currency]
display = "EUR"
convert = true

# Avvisi di prezzo (facoltativo)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Scorciatoie cripto personalizzate (facoltativo)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formati dei simboli

| Formato | Descrizione | Esempio |
|---------|-------------|---------|
| Azione | Ticker standard | `AAPL`, `GOOGL` |
| Cripto | Con suffisso USD | `BTC-USD`, `ETH-USD` |
| Abbreviazione cripto | Espanso in `-USD` | `BTC.X` -> `BTC-USD` |
| Cripto automatico | Criptovalute comuni | `BTC` -> `BTC-USD` |

## Fonte dei dati

Stonktop utilizza l'API di Yahoo Finance per recuperare le quotazioni in tempo reale. Non è richiesta alcuna chiave API.

## Sviluppo

### Compilazione

```bash
# Compilazione di debug
cargo build

# Compilazione release
cargo build --release
```

### Test

```bash
# Esegui tutti i test
cargo test

# Esegui i test mostrando l'output
cargo test -- --nocapture
```

### Analisi statica (Lint)

```bash
# Verifica eventuali problemi di linting
cargo clippy --all-targets --all-features

# Applica le correzioni automatiche
cargo clippy --fix --allow-dirty
```

### Formattazione

```bash
# Verifica la formattazione
cargo fmt -- --check

# Applica la formattazione
cargo fmt
```

## Contribuire

I contributi sono benvenuti! Sentiti libero di inviare una Pull Request.

1. Effettua un fork del repository
2. Crea il tuo branch di funzionalità (`git checkout -b feature/amazing-feature`)
3. Esegui il commit delle modifiche (`git commit -m 'Add amazing feature'`)
4. Effettua il push del branch (`git push origin feature/amazing-feature`)
5. Apri una Pull Request

## Licenza

Questo progetto è distribuito con licenza MIT — consulta il file [LICENSE](../../../LICENSE) per i dettagli.

## Ringraziamenti

- Ispirato a [ticker](https://github.com/achannarasappa/ticker)
- Realizzato con [ratatui](https://github.com/ratatui-org/ratatui)
- Dati forniti da [Yahoo Finance](https://finance.yahoo.com/)
