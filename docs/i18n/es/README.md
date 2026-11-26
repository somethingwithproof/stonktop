# Stonktop

> [English](../../../README.md)

Una interfaz de terminal inspirada en `top` para supervisar cotizaciones de acciones y criptomonedas en tiempo real.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Características

- **Supervisión de precios en tiempo real**: Cotizaciones en directo de acciones y criptomonedas
- **Interfaz similar a `top`**: Controles de teclado familiares análogos a `top`/`htop`
- **Gráficos sparkline**: Visualización integrada de tendencias de precios en una barra lateral
- **Seguimiento de cartera**: Controle sus posiciones con cálculos de pérdidas y ganancias (P/L)
- **Búsqueda y filtrado**: Búsqueda sin distinción entre mayúsculas y minúsculas por símbolos y nombres
- **Alertas de precio**: Notificaciones de escritorio cuando los precios cruzan los umbrales configurados
- **Exportación/Importación**: Guarde y cargue listas de seguimiento en formato JSON o CSV
- **Multidivisa**: Muestre los precios en cualquier divisa con tipos de cambio en tiempo real
- **Múltiples opciones de ordenación**: Ordene por símbolo, precio, variación, volumen o capitalización bursátil
- **Modo por lotes**: Salida no interactiva para scripts (similar a `top -b`)
- **Configuración TOML**: Archivos de configuración sencillos de editar
- **Codificación por colores**: Verde para ganancias, rojo para pérdidas
- **Atajos de criptomonedas**: Use `BTC.X` o simplemente `BTC` para `BTC-USD`

## Capturas de pantalla

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

## Requisitos previos

- **Rust**: Versión 1.88 o superior (MSRV)
- **Cargo**: Incluido con la instalación de Rust

## Instalación

### Desde crates.io (Recomendado)

```bash
cargo install stonktop
```

### Mediante cargo-binstall

Si dispone de [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), puede instalar binarios precompilados:

```bash
cargo binstall stonktop
```

### Binarios precompilados

Descargue los binarios precompilados desde la página de [Releases](https://github.com/somethingwithproof/stonktop/releases).

Plataformas disponibles:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### Desde el código fuente

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Uso

### Uso básico

```bash
# Supervisar símbolos específicos
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Con atajos de criptomonedas
stonktop -s AAPL,BTC,ETH,SOL
```

### Opciones similares a `top`

```bash
# Establecer el intervalo de actualización (como top -d)
stonktop -s AAPL,GOOGL -d 10

# Ejecutar N iteraciones y salir (como top -n)
stonktop -s AAPL -n 5

# Modo por lotes para scripts (como top -b)
stonktop -s AAPL,GOOGL -b

# Modo seguro: deshabilitar los comandos interactivos
stonktop -s AAPL -S
```

### Opciones de ordenación

```bash
# Ordenar por precio (descendente)
stonktop -s AAPL,GOOGL -o price

# Ordenar por símbolo (ascendente)
stonktop -s AAPL,GOOGL -o symbol -r

# Campos de ordenación disponibles: symbol, name, price, change, change-percent, volume, market-cap
```

### Archivo de configuración

```bash
# Usar un archivo de configuración específico
stonktop -c ~/.config/stonktop/custom.toml

# O colocar la configuración en la ubicación predeterminada:
# - Linux/macOS: ~/.config/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opciones de línea de comandos

| Opción | Forma corta | Descripción |
|--------|-------------|-------------|
| `--symbols` | `-s` | Lista separada por comas de símbolos a supervisar |
| `--delay` | `-d` | Intervalo de actualización en segundos (predeterminado: 5) |
| `--iterations` | `-n` | Número de iteraciones (0 = infinito) |
| `--batch` | `-b` | Modo por lotes: salida no interactiva |
| `--secure` | `-S` | Modo seguro: deshabilitar comandos interactivos |
| `--config` | `-c` | Ruta al archivo de configuración |
| `--sort` | `-o` | Campo de ordenación inicial |
| `--reverse` | `-r` | Invertir el orden de clasificación |
| `--holdings` | `-H` | Mostrar vista de posiciones/cartera |
| `--currency` | | Divisa de visualización (predeterminado: USD) |
| `--timeout` | | Tiempo de espera de la API en segundos (predeterminado: 10) |
| `--format` | | Formato de salida para el modo por lotes: `table`, `json`, `csv` |
| `--color` | | Salida en color: `auto`, `always`, `never` |
| `--verbose` | `-v` | Salida detallada |
| `--init` | | Generar un archivo de configuración de ejemplo |
| `--force` | | Forzar la sobrescritura de una configuración existente (con `--init`) |
| `--lang` | | Idioma de visualización (`STONKTOP_LANG`, predeterminado: automático desde `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Mostrar el mensaje de ayuda |
| `--version` | `-V` | Mostrar la versión |

## Idiomas compatibles

Stonktop detecta el idioma de visualización a partir de `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` y `LANG`, o acepta `--lang <LANG>`. Las 45 configuraciones regionales compatibles incluyen traducciones completas de la CLI y la TUI (incluida la alineación de derecha a izquierda para árabe, persa, hebreo y urdu) y documentación localizada:

| Código | Idioma | README | Código | Idioma | README | Código | Idioma | README |
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

## Atajos de teclado

| Tecla | Acción |
|-------|--------|
| `q`, `Esc` | Salir |
| `h`, `?` | Mostrar/ocultar la ayuda |
| `↑`, `k` | Subir |
| `↓`, `j` | Bajar |
| `g`, `Home` | Ir al principio |
| `G`, `End` | Ir al final |
| `PgUp` | Página anterior |
| `PgDn` | Página siguiente |
| `s` | Cambiar el campo de ordenación |
| `r` | Invertir el orden de clasificación |
| `1-7` | Ordenar por columna |
| `Enter`, `d` | Mostrar ventana de detalles del símbolo seleccionado |
| `/` | Buscar/filtrar símbolos |
| `a` | Añadir un símbolo a la lista de seguimiento |
| `x` | Eliminar el símbolo seleccionado |
| `S` | Mostrar/ocultar la barra lateral de gráficos sparkline |
| `e` | Exportar la lista de seguimiento a JSON |
| `H` | Mostrar/ocultar la vista de posiciones |
| `f` | Mostrar/ocultar los datos fundamentales |
| `Space`, `R` | Forzar actualización |
| `Tab` | Cambiar entre grupos de símbolos |

## Configuración

Cree un archivo de configuración TOML:

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

# Posiciones de la cartera (opcional)
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

# Grupos de símbolos
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]

# Visualización multidivisa (opcional)
[currency]
display = "EUR"
convert = true

# Alertas de precio (opcional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Atajos personalizados de criptomonedas (opcional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formatos de símbolos

| Formato | Descripción | Ejemplo |
|---------|-------------|---------|
| Acción | Símbolo bursátil estándar | `AAPL`, `GOOGL` |
| Criptomoneda | Con sufijo USD | `BTC-USD`, `ETH-USD` |
| Abreviatura cripto | Se expande a `-USD` | `BTC.X` -> `BTC-USD` |
| Cripto automático | Criptomonedas habituales | `BTC` -> `BTC-USD` |

## Fuente de datos

Stonktop utiliza la API de Yahoo Finance para obtener cotizaciones en tiempo real. No se requiere ninguna clave de API.

## Desarrollo

### Compilación

```bash
# Compilación de depuración
cargo build

# Compilación de producción
cargo build --release
```

### Pruebas

```bash
# Ejecutar todas las pruebas
cargo test

# Ejecutar las pruebas mostrando la salida
cargo test -- --nocapture
```

### Análisis estático (Lint)

```bash
# Comprobar problemas de estilo y análisis estático
cargo clippy --all-targets --all-features

# Aplicar correcciones automáticas
cargo clippy --fix --allow-dirty
```

### Formateo

```bash
# Comprobar el formato del código
cargo fmt -- --check

# Aplicar el formato
cargo fmt
```

## Contribuciones

Las contribuciones son bienvenidas. No dude en enviar un Pull Request.

1. Haga un fork del repositorio
2. Cree su rama de funcionalidad (`git checkout -b feature/amazing-feature`)
3. Confirme sus cambios (`git commit -m 'Add amazing feature'`)
4. Envíe la rama al repositorio remoto (`git push origin feature/amazing-feature`)
5. Abra un Pull Request

## Licencia

Este proyecto se distribuye bajo la licencia MIT; consulte el archivo [LICENSE](../../../LICENSE) para obtener más detalles.

## Agradecimientos

- Inspirado en [ticker](https://github.com/achannarasappa/ticker)
- Desarrollado con [ratatui](https://github.com/ratatui-org/ratatui)
- Datos proporcionados por [Yahoo Finance](https://finance.yahoo.com/)
