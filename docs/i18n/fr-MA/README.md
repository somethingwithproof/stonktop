# Stonktop

> [English](../../../README.md)

Une interface terminal de type `top` pour suivre en temps réel les cours des actions et des cryptomonnaies.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Fonctionnalités

- **Suivi des cours en temps réel** : cotations en direct des valeurs boursières et des cryptomonnaies
- **Interface de type `top`** : navigation au clavier familière, proche de `top`/`htop`
- **Graphiques sparkline** : visualisation instantanée de la tendance des cours dans un volet latéral
- **Suivi de portefeuille** : suivi de vos positions avec calcul des plus-values et moins-values (P/L)
- **Recherche et filtrage** : recherche insensible à la casse parmi les symboles et les dénominations
- **Alertes de cours** : notifications sur le bureau lorsque les cours franchissent les seuils définis
- **Export et import** : sauvegarde et chargement des listes de suivi aux formats JSON ou CSV
- **Multidevise** : affichage des prix dans la devise de votre choix avec conversion au taux de change en direct
- **Options de tri multiples** : tri par symbole, cours, variation, volume d'échanges ou capitalisation boursière
- **Mode lot (batch)** : sortie non interactive adaptée à l'automatisation par scripts (comme `top -b`)
- **Configuration TOML** : fichiers de paramétrage clairs et simples à éditer
- **Code couleur** : vert pour les hausses, rouge pour les baisses
- **Raccourcis crypto** : saisissez `BTC.X` ou simplement `BTC` pour `BTC-USD`

## Captures d'écran

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

## Prérequis

- **Rust** : version 1.89 ou supérieure (MSRV)
- **Cargo** : inclus avec l'installation de Rust

## Installation

### Depuis crates.io (Recommandé)

```bash
cargo install stonktop
```

### Avec cargo-binstall

Si [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) est déjà installé sur votre poste, vous pouvez installer directement les binaires précompilés :

```bash
cargo binstall stonktop
```

### Binaires précompilés

Téléchargez les binaires précompilés depuis la page [Releases](https://github.com/somethingwithproof/stonktop/releases).

Plateformes disponibles :
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Compilation depuis les sources

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Utilisation

### Utilisation de base

```bash
# Suivre des symboles précis
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Avec les raccourcis crypto
stonktop -s AAPL,BTC,ETH,SOL
```

### Options de type `top`

```bash
# Définir l'intervalle d'actualisation (comme top -d)
stonktop -s AAPL,GOOGL -d 10

# Exécuter N itérations puis quitter (comme top -n)
stonktop -s AAPL -n 5

# Mode lot pour les scripts (comme top -b)
stonktop -s AAPL,GOOGL -b

# Mode sécurisé : désactive les commandes interactives
stonktop -s AAPL -S
```

### Options de tri

```bash
# Trier par cours (décroissant)
stonktop -s AAPL,GOOGL -o price

# Trier par symbole (croissant)
stonktop -s AAPL,GOOGL -o symbol -r

# Champs de tri disponibles : symbol, name, price, change, change-percent, volume, market-cap
```

### Fichier de configuration

```bash
# Utiliser un fichier de configuration spécifique
stonktop -c ~/.config/stonktop/custom.toml

# Ou placer le fichier à l'emplacement par défaut :
# - Linux : ~/.config/stonktop/config.toml
# - macOS : ~/Library/Application Support/stonktop/config.toml
# - Windows : %APPDATA%\stonktop\config.toml
```

## Options de ligne de commande

| Option | Raccourci | Description |
|--------|-----------|-------------|
| `--symbols` | `-s` | Liste de symboles à suivre, séparés par des virgules |
| `--delay` | `-d` | Délai d'actualisation en secondes (par défaut : 5) |
| `--iterations` | `-n` | Nombre d'itérations (`0` = illimité) |
| `--batch` | `-b` | Mode lot : sortie non interactive |
| `--secure` | `-S` | Mode sécurisé : désactive les commandes interactives |
| `--config` | `-c` | Chemin vers le fichier de configuration |
| `--sort` | `-o` | Champ de tri initial |
| `--reverse` | `-r` | Inverser l'ordre de tri |
| `--holdings` | `-H` | Afficher la vue portefeuille / positions |
| `--currency` | | Devise d'affichage (par défaut : `USD`) |
| `--timeout` | | Délai d'expiration de l'API en secondes (par défaut : 10) |
| `--format` | | Format de sortie en mode lot : `table`, `json`, `csv` |
| `--color` | | Coloration de la sortie : `auto`, `always`, `never` |
| `--verbose` | `-v` | Sortie détaillée |
| `--init` | | Générer un fichier de configuration exemple |
| `--force` | | Écraser une configuration existante (avec `--init`) |
| `--lang` | | Langue d'affichage (`STONKTOP_LANG`, par défaut : détection automatique via `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Afficher l'aide |
| `--version` | `-V` | Afficher la version |

## Langues prises en charge

Stonktop détecte automatiquement la langue d'affichage à partir des variables `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` et `LANG`, ou accepte l'option `--lang <LANG>`. Les 48 locales prises en charge incluent une traduction complète de l'interface CLI et TUI (y compris l'alignement de droite à gauche pour l'arabe, le persan, l'hébreu et l'ourdou) ainsi qu'une documentation dédiée :

| Code | Langue | README | Code | Langue | README | Code | Langue | README |
|------|--------|--------|------|--------|--------|------|--------|--------|
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

## Raccourcis clavier

| Touche | Action |
|--------|--------|
| `q`, `Esc` | Quitter |
| `h`, `?` | Afficher ou masquer l'aide |
| `↑`, `k` | Monter |
| `↓`, `j` | Descendre |
| `g`, `Home` | Aller en haut de la liste |
| `G`, `End` | Aller en bas de la liste |
| `PgUp` | Page précédente |
| `PgDn` | Page suivante |
| `s` | Changer le critère de tri |
| `r` | Inverser l'ordre de tri |
| `1-7` | Trier par colonne |
| `Enter` | Ouvrir la fenêtre détaillée du symbole sélectionné |
| `/` | Rechercher / filtrer les symboles |
| `a` | Ajouter un symbole à la liste de suivi |
| `d` | Supprimer le symbole sélectionné |
| `S` | Afficher ou masquer le volet latéral des graphiques sparkline |
| `e` | Exporter la liste de suivi au format JSON |
| `H` | Basculer vers la vue portefeuille |
| `f` | Afficher ou masquer les données fondamentales |
| `Space`, `R` | Forcer l'actualisation |
| `Tab` | Parcourir les groupes de symboles |

## Configuration

Créez un fichier de configuration au format TOML :

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

# Positions en portefeuille (facultatif)
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

# Groupes de symboles
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Affichage multidevise (facultatif)
[currency]
display = "EUR"
convert = true

# Alertes de cours (facultatif)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Raccourcis crypto personnalisés (facultatif)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formats de symboles

| Format | Description | Exemple |
|--------|-------------|---------|
| Action | Code boursier (ticker) standard | `AAPL`, `GOOGL` |
| Crypto | Avec suffixe `USD` | `BTC-USD`, `ETH-USD` |
| Abréviation crypto | Développé automatiquement en `-USD` | `BTC.X` -> `BTC-USD` |
| Crypto automatique | Cryptomonnaies courantes | `BTC` -> `BTC-USD` |

## Source des données

Stonktop s'appuie sur l'API Yahoo Finance pour récupérer les cotations en temps réel. Aucune clé API n'est requise.

## Développement

### Compilation

```bash
# Compilation en mode debug
cargo build

# Compilation en mode release
cargo build --release
```

### Tests

```bash
# Exécuter l'ensemble des tests
cargo test

# Exécuter les tests avec affichage complet des sorties
cargo test -- --nocapture
```

### Analyse statique (Lint)

```bash
# Vérifier les avertissements et problèmes de lint
cargo clippy --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty
```

### Formatage

```bash
# Vérifier le formatage du code
cargo fmt -- --check

# Appliquer le formatage
cargo fmt
```

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à soumettre une Pull Request.

1. Forkez le dépôt
2. Créez votre branche de fonctionnalité (`git checkout -b feature/amazing-feature`)
3. Validez vos modifications (`git commit -m 'Add amazing feature'`)
4. Poussez la branche vers le dépôt distant (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request

## Licence

Ce projet est distribué sous licence MIT ; consultez le fichier [LICENSE](../../../LICENSE) pour plus de détails.

## Remerciements

- Inspiré par [ticker](https://github.com/achannarasappa/ticker)
- Développé avec [ratatui](https://github.com/ratatui-org/ratatui)
- Données fournies par [Yahoo Finance](https://finance.yahoo.com/)
