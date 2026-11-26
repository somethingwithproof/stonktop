# Stonktop

> [English](../../../README.md)

Une interface terminal de type `top` pour suivre les cours des actions et des cryptomonnaies en temps réel.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Fonctionnalités

- **Suivi des cours en temps réel** : Cotations en direct pour les actions et les cryptomonnaies
- **Interface de type `top`** : Commandes clavier familières similaires à `top`/`htop`
- **Graphiques sparkline** : Visualisation intégrée des tendances de prix dans une barre latérale
- **Suivi de portefeuille** : Suivez vos positions avec calcul des gains et pertes (P/L)
- **Recherche et filtrage** : Recherche insensible à la casse sur les symboles et les noms
- **Alertes de prix** : Notifications de bureau lorsque les cours franchissent les seuils configurés
- **Exportation/Importation** : Enregistrez et chargez vos listes de suivi aux formats JSON ou CSV
- **Multi-devises** : Affichez les prix dans n'importe quelle devise avec conversion en direct
- **Options de tri multiples** : Triez par symbole, prix, variation, volume ou capitalisation boursière
- **Mode batch** : Sortie non interactive adaptée aux scripts (comme `top -b`)
- **Configuration TOML** : Fichiers de configuration simples à éditer
- **Code couleur** : Vert pour les hausses, rouge pour les baisses
- **Raccourcis crypto** : Utilisez `BTC.X` ou simplement `BTC` pour `BTC-USD`

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

- **Rust** : Version 1.88 ou supérieure (MSRV)
- **Cargo** : Inclus avec l'installation de Rust

## Installation

### Depuis crates.io (Recommandé)

```bash
cargo install stonktop
```

### Avec cargo-binstall

Si [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) est installé, vous pouvez installer des binaires précompilés :

```bash
cargo binstall stonktop
```

### Binaires précompilés

Téléchargez les binaires précompilés depuis la page [Releases](https://github.com/somethingwithproof/stonktop/releases).

Plateformes disponibles :
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Depuis les sources

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
# Définir le délai d'actualisation (comme top -d)
stonktop -s AAPL,GOOGL -d 10

# Exécuter N itérations puis quitter (comme top -n)
stonktop -s AAPL -n 5

# Mode batch pour les scripts (comme top -b)
stonktop -s AAPL,GOOGL -b

# Mode sécurisé : désactiver les commandes interactives
stonktop -s AAPL -S
```

### Options de tri

```bash
# Trier par prix (décroissant)
stonktop -s AAPL,GOOGL -o price

# Trier par symbole (croissant)
stonktop -s AAPL,GOOGL -o symbol -r

# Champs de tri disponibles : symbol, name, price, change, change-percent, volume, market-cap
```

### Fichier de configuration

```bash
# Utiliser un fichier de configuration précis
stonktop -c ~/.config/stonktop/custom.toml

# Ou placer la configuration à l'emplacement par défaut :
# - Linux/macOS : ~/.config/stonktop/config.toml
# - Windows : %APPDATA%\stonktop\config.toml
```

## Options de ligne de commande

| Option | Forme courte | Description |
|--------|--------------|-------------|
| `--symbols` | `-s` | Liste de symboles à suivre séparés par des virgules |
| `--delay` | `-d` | Délai d'actualisation en secondes (par défaut : 5) |
| `--iterations` | `-n` | Nombre d'itérations (0 = infini) |
| `--batch` | `-b` | Mode batch — sortie non interactive |
| `--secure` | `-S` | Mode sécurisé — désactiver les commandes interactives |
| `--config` | `-c` | Chemin vers le fichier de configuration |
| `--sort` | `-o` | Champ de tri initial |
| `--reverse` | `-r` | Inverser l'ordre de tri |
| `--holdings` | `-H` | Afficher la vue des positions/portefeuille |
| `--currency` | | Devise d'affichage (par défaut : USD) |
| `--timeout` | | Délai d'expiration de l'API en secondes (par défaut : 10) |
| `--format` | | Format de sortie pour le mode batch : `table`, `json`, `csv` |
| `--color` | | Sortie en couleur : `auto`, `always`, `never` |
| `--verbose` | `-v` | Sortie détaillée |
| `--init` | | Générer un fichier de configuration d'exemple |
| `--force` | | Forcer l'écrasement d'une configuration existante (avec `--init`) |
| `--lang` | | Langue d'affichage (`STONKTOP_LANG`, par défaut : détection automatique depuis `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Afficher le message d'aide |
| `--version` | `-V` | Afficher la version |

## Langues prises en charge

Stonktop détecte la langue d'affichage à partir de `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` et `LANG`, ou accepte `--lang <LANG>`. Les 45 paramètres régionaux pris en charge incluent des traductions complètes pour la CLI et la TUI (y compris l'alignement de droite à gauche pour l'arabe, le persan, l'hébreu et l'ourdou) ainsi qu'une documentation localisée :

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

## Raccourcis clavier

| Touche | Action |
|--------|--------|
| `q`, `Esc` | Quitter |
| `h`, `?` | Afficher/masquer l'aide |
| `↑`, `k` | Monter |
| `↓`, `j` | Descendre |
| `g`, `Home` | Aller en haut |
| `G`, `End` | Aller en bas |
| `PgUp` | Page précédente |
| `PgDn` | Page suivante |
| `s` | Changer de champ de tri |
| `r` | Inverser l'ordre de tri |
| `1-7` | Trier par colonne |
| `Enter`, `d` | Afficher la fenêtre de détails du symbole sélectionné |
| `/` | Rechercher/filtrer des symboles |
| `a` | Ajouter un symbole à la liste de suivi |
| `x` | Supprimer le symbole sélectionné |
| `S` | Afficher/masquer la barre latérale de graphiques sparkline |
| `e` | Exporter la liste de suivi en JSON |
| `H` | Afficher/masquer la vue des positions |
| `f` | Afficher/masquer les données fondamentales |
| `Space`, `R` | Forcer l'actualisation |
| `Tab` | Parcourir les groupes de symboles |

## Configuration

Créez un fichier de configuration TOML :

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

# Positions du portefeuille (facultatif)
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

# Affichage multi-devises (facultatif)
[currency]
display = "EUR"
convert = true

# Alertes de prix (facultatif)
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
| Action | Symbole boursier standard | `AAPL`, `GOOGL` |
| Crypto | Avec suffixe USD | `BTC-USD`, `ETH-USD` |
| Abréviation crypto | Développé en `-USD` | `BTC.X` -> `BTC-USD` |
| Crypto automatique | Cryptomonnaies courantes | `BTC` -> `BTC-USD` |

## Source de données

Stonktop utilise l'API Yahoo Finance pour récupérer les cotations en temps réel. Aucune clé d'API n'est nécessaire.

## Développement

### Compilation

```bash
# Compilation de débogage
cargo build

# Compilation release
cargo build --release
```

### Tests

```bash
# Exécuter tous les tests
cargo test

# Exécuter les tests avec affichage de la sortie
cargo test -- --nocapture
```

### Analyse statique (Lint)

```bash
# Vérifier les problèmes d'analyse statique
cargo clippy --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty
```

### Formatage

```bash
# Vérifier le formatage
cargo fmt -- --check

# Appliquer le formatage
cargo fmt
```

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à soumettre une Pull Request.

1. Forkez le dépôt
2. Créez votre branche de fonctionnalité (`git checkout -b feature/amazing-feature`)
3. Commitez vos modifications (`git commit -m 'Add amazing feature'`)
4. Poussez la branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request

## Licence

Ce projet est distribué sous licence MIT — consultez le fichier [LICENSE](../../../LICENSE) pour plus de détails.

## Remerciements

- Inspiré par [ticker](https://github.com/achannarasappa/ticker)
- Développé avec [ratatui](https://github.com/ratatui-org/ratatui)
- Données fournies par [Yahoo Finance](https://finance.yahoo.com/)
