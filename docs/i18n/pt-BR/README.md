# Stonktop

> [English](../../../README.md)

Uma interface de terminal inspirada no `top` para monitorar cotações de ações e criptomoedas em tempo real.

[![Crates.io](https://img.shields.io/crates/v/stonktop.svg)](https://crates.io/crates/stonktop)
[![Downloads](https://img.shields.io/crates/d/stonktop.svg)](https://crates.io/crates/stonktop)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/somethingwithproof/stonktop/workflows/CI/badge.svg)](https://github.com/somethingwithproof/stonktop/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/stonktop)](https://github.com/somethingwithproof/stonktop/releases)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/stonktop/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/stonktop)

## Recursos

- **Monitoramento de preços em tempo real**: Cotações ao vivo para ações e criptomoedas
- **Interface no estilo `top`**: Controles de teclado familiares semelhantes ao `top`/`htop`
- **Gráficos sparkline**: Visualização embutida de tendências de preço em uma barra lateral
- **Acompanhamento de portfólio**: Monitore suas posições com cálculos de lucro e prejuízo (P/L)
- **Busca e filtragem**: Busca sem diferenciação entre maiúsculas e minúsculas por símbolos e nomes
- **Alertas de preço**: Notificações na área de trabalho quando os preços cruzam limites configurados
- **Exportação/Importação**: Salve e carregue listas de acompanhamento em JSON ou CSV
- **Multimoeda**: Exiba preços em qualquer moeda com taxas de conversão em tempo real
- **Múltiplas opções de ordenação**: Ordene por símbolo, preço, variação, volume ou valor de mercado
- **Modo em lote (Batch)**: Saída não interativa para scripts (como `top -b`)
- **Configuração em TOML**: Arquivos de configuração fáceis de editar
- **Codificação por cores**: Verde para altas, vermelho para baixas
- **Atalhos para criptomoedas**: Use `BTC.X` ou apenas `BTC` para `BTC-USD`

## Capturas de tela

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

## Pré-requisitos

- **Rust**: Versão 1.89 ou superior (MSRV)
- **Cargo**: Incluído na instalação do Rust

## Instalação

### Via crates.io (Recomendado)

```bash
cargo install stonktop
```

### Usando cargo-binstall

Se você tiver o [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) instalado, poderá instalar binários pré-compilados:

```bash
cargo binstall stonktop
```

### Binários pré-compilados

Baixe os binários pré-compilados na página de [Releases](https://github.com/somethingwithproof/stonktop/releases).

Plataformas disponíveis:
- Linux (x86_64, aarch64, musl)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### A partir do código-fonte

```bash
git clone https://github.com/somethingwithproof/stonktop.git
cd stonktop
cargo build --release
cargo install --path .
```

## Uso

### Uso básico

```bash
# Monitorar símbolos específicos
stonktop -s AAPL,GOOGL,MSFT,BTC-USD

# Com atalhos de criptomoedas
stonktop -s AAPL,BTC,ETH,SOL
```

### Opções no estilo `top`

```bash
# Definir o intervalo de atualização (como top -d)
stonktop -s AAPL,GOOGL -d 10

# Executar N iterações e sair (como top -n)
stonktop -s AAPL -n 5

# Modo em lote para scripts (como top -b)
stonktop -s AAPL,GOOGL -b

# Modo seguro - desativar comandos interativos
stonktop -s AAPL -S
```

### Opções de ordenação

```bash
# Ordenar por preço (decrescente)
stonktop -s AAPL,GOOGL -o price

# Ordenar por símbolo (crescente)
stonktop -s AAPL,GOOGL -o symbol -r

# Campos de ordenação disponíveis: symbol, name, price, change, change-percent, volume, market-cap
```

### Arquivo de configuração

```bash
# Usar um arquivo de configuração específico
stonktop -c ~/.config/stonktop/custom.toml

# Ou colocar a configuração no local padrão:
# - Linux: ~/.config/stonktop/config.toml
# - macOS: ~/Library/Application Support/stonktop/config.toml
# - Windows: %APPDATA%\stonktop\config.toml
```

## Opções de linha de comando

| Opção | Forma curta | Descrição |
|-------|-------------|-----------|
| `--symbols` | `-s` | Lista de símbolos para monitorar separados por vírgula |
| `--delay` | `-d` | Intervalo de atualização em segundos (padrão: 5) |
| `--iterations` | `-n` | Número de iterações (0 = infinito) |
| `--batch` | `-b` | Modo em lote — saída não interativa |
| `--secure` | `-S` | Modo seguro — desativa comandos interativos |
| `--config` | `-c` | Caminho para o arquivo de configuração |
| `--sort` | `-o` | Campo de ordenação inicial |
| `--reverse` | `-r` | Inverter a ordem de classificação |
| `--holdings` | `-H` | Exibir visualização de posições/portfólio |
| `--currency` | | Moeda de exibição (padrão: USD) |
| `--timeout` | | Tempo limite da API em segundos (padrão: 10) |
| `--format` | | Formato de saída para o modo em lote: `table`, `json`, `csv` |
| `--color` | | Saída colorida: `auto`, `always`, `never` |
| `--verbose` | `-v` | Saída detalhada |
| `--init` | | Gerar um arquivo de configuração de exemplo |
| `--force` | | Forçar a sobrescrita da configuração existente (com `--init`) |
| `--lang` | | Idioma de exibição (`STONKTOP_LANG`, padrão: automático a partir de `LC_ALL`/`LC_MESSAGES`/`LANG`) |
| `--help` | | Exibir mensagem de ajuda |
| `--version` | `-V` | Exibir versão |

## Idiomas suportados

O Stonktop detecta o idioma de exibição a partir de `STONKTOP_LANG`, `LC_ALL`, `LC_MESSAGES` e `LANG`, ou aceita `--lang <LANG>`. Todas as 45 localidades suportadas incluem traduções completas da CLI e da TUI (incluindo alinhamento da direita para a esquerda para árabe, persa, hebraico e urdu) e documentação localizada:

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

## Atalhos de teclado

| Tecla | Ação |
|-------|------|
| `q`, `Esc` | Sair |
| `h`, `?` | Alternar ajuda |
| `↑`, `k` | Mover para cima |
| `↓`, `j` | Mover para baixo |
| `g`, `Home` | Ir para o topo |
| `G`, `End` | Ir para o final |
| `PgUp` | Página anterior |
| `PgDn` | Próxima página |
| `s` | Alternar campo de ordenação |
| `r` | Inverter ordem de classificação |
| `1-7` | Ordenar por coluna |
| `Enter` | Exibir janela de detalhes do símbolo selecionado |
| `/` | Buscar/filtrar símbolos |
| `a` | Adicionar um símbolo à lista de acompanhamento |
| `d` | Remover símbolo selecionado |
| `S` | Alternar barra lateral de gráficos sparkline |
| `e` | Exportar lista de acompanhamento para JSON |
| `H` | Alternar visualização de posições |
| `f` | Alternar dados fundamentalistas |
| `Space`, `R` | Forçar atualização |
| `Tab` | Alternar grupos de símbolos |

## Configuração

Crie um arquivo de configuração TOML:

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

# Posições do portfólio (opcional)
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

# Exibição multimoeda (opcional)
[currency]
display = "EUR"
convert = true

# Alertas de preço (opcional)
[[alerts]]
symbol = "AAPL"
above = 200.00

[[alerts]]
symbol = "BTC-USD"
below = 20000.00

# Atalhos personalizados de criptomoedas (opcional)
[shortcuts]
PEPE = "PEPE-USD"
SHIB = "SHIB-USD"
```

## Formatos de símbolos

| Formato | Descrição | Exemplo |
|---------|-----------|---------|
| Ação | Código de negociação padrão | `AAPL`, `GOOGL` |
| Cripto | Com sufixo USD | `BTC-USD`, `ETH-USD` |
| Abreviação cripto | Expandido para `-USD` | `BTC.X` -> `BTC-USD` |
| Cripto automático | Criptomoedas comuns | `BTC` -> `BTC-USD` |

## Fonte de dados

O Stonktop utiliza a API do Yahoo Finance para obter cotações em tempo real. Nenhuma chave de API é necessária.

## Desenvolvimento

### Compilação

```bash
# Compilação de depuração
cargo build

# Compilação de produção
cargo build --release
```

### Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com saída completa
cargo test -- --nocapture
```

### Lint

```bash
# Verificar problemas de lint
cargo clippy --all-targets --all-features

# Aplicar correções automáticas
cargo clippy --fix --allow-dirty
```

### Formatação

```bash
# Verificar formatação
cargo fmt -- --check

# Aplicar formatação
cargo fmt
```

## Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para enviar um Pull Request.

1. Faça um fork do repositório
2. Crie sua branch de recurso (`git checkout -b feature/amazing-feature`)
3. Faça commit das suas alterações (`git commit -m 'Add amazing feature'`)
4. Envie a branch (`git push origin feature/amazing-feature`)
5. Abra um Pull Request

## Licença

Este projeto é licenciado sob a Licença MIT — consulte o arquivo [LICENSE](../../../LICENSE) para mais detalhes.

## Agradecimentos

- Inspirado no [ticker](https://github.com/achannarasappa/ticker)
- Construído com [ratatui](https://github.com/ratatui-org/ratatui)
- Dados fornecidos pelo [Yahoo Finance](https://finance.yahoo.com/)
