# Stonktop

> [English](../../../README.md)

Une interface en mode terminal inspirée de `top` pour surveiller en temps réel les cours des actions et des cryptomonnaies.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Fonctionnalités

- **Suivi des prix en temps réel** : cotations actualisées en direct pour les actions et les cryptomonnaies
- **Interface similaire à `top`** : commandes clavier intuitives proches de `top`/`htop`
- **Graphiques sparkline** : suivi visuel de l'évolution des cours dans un panneau latéral
- **Gestion de portefeuille** : suivi de vos titres détenus avec calcul des gains et pertes (P/L)
- **Recherche et filtrage** : recherche insensible à la casse sur les symboles et les noms de sociétés
- **Alertes de prix** : notifications de bureau dès qu'un cours dépasse les seuils configurés
- **Exportation / Importation** : enregistrement et chargement des listes de surveillance en JSON ou CSV
- **Multidevise** : affichage des montants dans n'importe quelle devise avec taux de conversion en direct
- **Multiples critères de tri** : tri par symbole, prix, variation, volume de transactions ou capitalisation boursière
- **Mode batch** : sortie non interactive conçue pour les scripts d'automatisation (comme `top -b`)
- **Configuration TOML** : fichiers de configuration lisibles et faciles à modifier
- **Code couleur** : vert pour les gains, rouge pour les pertes
- **Raccourcis cryptomonnaies** : utilisez `BTC.X` ou directement `BTC` pour `BTC-USD`

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

- **Rust** : version 1.88 ou plus récente (MSRV)
- **Cargo** : fourni automatiquement avec Rust

## Installation

### Via crates.io (Recommandé)

```bash
cargo install stonktop
```

### Avec cargo-binstall

Si vous disposez de [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), vous pouvez récupérer directement des exécutables précompilés :

```bash
cargo binstall stonktop
```

### Binaires précompilés

Téléchargez les binaires prêts à l'emploi depuis la page [Releases](https://github.com/somethingwithproof/stonktop/releases).

Plateformes prises en charge :
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### À partir du code source

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Utilisation

### Utilisation courante

```bash
# Surveiller des symboles donnés
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Utiliser les raccourcis crypto
stonktop -s AAPL,BTC,ETH,SOL
```

### Options inspirées de `top`

```bash
# Fixer le délai de rafraîchissement (comme top -d)
stonktop -s AAPL,GOOGL -d 10

# Exécuter N cycles d'actualisation puis quitter (comme top -n)
stonktop -s AAPL -n 5

# Mode batch pour les scripts (comme top -b)
stonktop -s AAPL,GOOGL -b

# Mode sécurisé : désactiver les commandes interactives
stonktop -s AAPL -S
```

### Options de tri

```bash
# Trier par prix (ordre décroissant)
stonktop -s AAPL,GOOGL -o price

# Trier par symbole (ordre croissant)
stonktop -s AAPL,GOOGL -o symbol -r

# Champs de tri disponibles : symbol, name, price, change, change-percent, volume, market-cap
```

### Fichier de configuration

```bash
# Indiquer un fichier de configuration personnalisé
stonktop -c ~/.config/stonktop/custom.toml

# Ou enregistrer le fichier à l'emplacement par défaut :
# - Linux/macOS : ~/.config/stonktop/config.toml
# - Windows : %APPDATA%\stonktop\config.toml
```

## Options de la ligne de commande

| Option | Forme courte | Description |
|--------|--------------|-------------|
| `--symbols` | `-s` | Liste des symboles à surveiller, séparés par des virgules |
| `--delay` | `-d` | Intervalle de rafraîchissement en secondes (par défaut : 5) |
| `--iterations` | `-n` | Nombre d'itérations (`0` = infini) |
| `--batch` | `-b` | Mode batch : sortie non interactive |
| `--secure` | `-S` | Mode sécurisé : désactive les commandes interactives |
| `--config` | `-c` | Chemin d'accès au fichier de configuration |
| `--sort` | `-o` | Champ de tri au démarrage |
| `--reverse` | `-r` | Inverser le sens du tri |
| `--holdings` | `-H` | Afficher la vue portefeuille / avoirs |
| `--currency` | | Devise d'affichage (par défaut : `USD`) |
| `--timeout` | | Délai d'attente maximal de l'API en secondes (par défaut : 10) |
| `--format` | | Format de sortie en mode batch : `table`, `json`, `csv` |
| `--color` | | Gestion des couleurs : `auto`, `always`, `never` |
| `--verbose` | `-v` | Affichage verbeux |
| `--init` | | Créer un fichier de configuration d'exemple |
| `--force` | | Remplacer le fichier existant (avec `--init`) |
| `--lang` | | Langue de l'interface (`STONKTOP_LANG`, par défaut : détectée depuis `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Afficher le message d'aide |
| `--version` | `-V` | Afficher le numéro de version |

## Langues prises en charge

Stonktop détermine la langue d'affichage à partir de `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` et `LANG`, ou via l'option `--lang <LANG>`. Chacune des 45 locales prises en charge bénéficie d'une traduction complète de l'interface CLI et TUI (incluant l'alignement de droite à gauche pour l'arabe, le persan, l'hébreu et l'ourdou) ainsi que de sa documentation dédiée :

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
| `h`, `?` | Afficher ou masquer l'aide |
| `↑`, `k` | Aller vers le haut |
| `↓`, `j` | Aller vers le bas |
| `g`, `Home` | Revenir au début de la liste |
| `G`, `End` | Aller à la fin de la liste |
| `PgUp` | Page précédente |
| `PgDn` | Page suivante |
| `s` | Faire défiler les champs de tri |
| `r` | Inverser l'ordre de tri |
| `1-7` | Trier selon la colonne |
| `Enter`, `d` | Afficher la fiche détaillée du symbole sélectionné |
| `/` | Rechercher / filtrer les symboles |
| `a` | Ajouter un symbole à la liste de surveillance |
| `x` | Retirer le symbole sélectionné |
| `S` | Afficher ou masquer le panneau latéral des graphiques sparkline |
| `e` | Exporter la liste de surveillance en JSON |
| `H` | Basculer l'affichage du portefeuille |
| `f` | Afficher ou masquer les indicateurs fondamentaux |
| `Space`, `R` | Forcer le rafraîchissement |
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

# Titres détenus en portefeuille (optionnel)
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

# Affichage dans une autre devise (optionnel)
[currency]
display = "EUR"
convert = true

# Alertes de prix (optionnel)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Raccourcis crypto personnalisés (optionnel)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formats de symboles

| Format | Description | Exemple |
|--------|-------------|---------|
| Action | Symbole boursier standard | `AAPL`, `GOOGL` |
| Crypto | Avec suffixe `USD` | `BTC-USD`, `ETH-USD` |
| Raccourci crypto | Converti en `-USD` | `BTC.X` -> `BTC-USD` |
| Crypto automatique | Cryptomonnaies courantes | `BTC` -> `BTC-USD` |

## Source des données

Stonktop utilise l'API Yahoo Finance pour obtenir les cotations en temps réel. Aucune clé API n'est nécessaire.

## Développement

### Compilation

```bash
# Compilation debug
cargo build

# Compilation release
cargo build --release
```

### Tests

```bash
# Lancer tous les tests
cargo test

# Lancer les tests en affichant les sorties
cargo test -- --nocapture
```

### Vérification (Lint)

```bash
# Vérifier les avertissements du linter
cargo clippy --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty
```

### Formatage

```bash
# Contrôler le formatage
cargo fmt -- --check

# Appliquer le formatage
cargo fmt
```

## Contribuer

Vos contributions sont les bienvenues ! N'hésitez pas à proposer une Pull Request.

1. Créez un fork du dépôt
2. Créez votre branche de fonctionnalité (`git checkout -b feature/amazing-feature`)
3. Enregistrez vos changements (`git commit -m 'Add amazing feature'`)
4. Poussez votre branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request

## Licence

Ce projet est publié sous licence MIT — reportez-vous au fichier [LICENSE](../../../LICENSE) pour tous les détails.

## Remerciements

- Inspiré par [ticker](https://github.com/achannarasappa/ticker)
- Construit avec [ratatui](https://github.com/ratatui-org/ratatui)
- Données issues de [Yahoo Finance](https://finance.yahoo.com/)
