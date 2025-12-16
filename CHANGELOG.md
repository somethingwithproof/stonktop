# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-16

### Added
- Initial release
- Real-time stock and cryptocurrency price monitoring
- Top-like terminal interface with familiar keyboard shortcuts
- Support for Yahoo Finance API (no API key required)
- Portfolio/holdings tracking with P/L calculations
- Multiple sort options (symbol, price, change, volume, market cap)
- TOML configuration file support
- Batch mode for scripting (`-b` flag)
- Secure mode to disable interactive commands (`-S` flag)
- Crypto symbol shortcuts (BTC.X -> BTC-USD)
- Configurable refresh interval (`-d` flag)
- Iteration limit (`-n` flag)
- Color-coded gains (green) and losses (red)
- Vim-style navigation (j/k, g/G)
- Help overlay (h/?)
- Performance data display

### Platforms
- Linux (x86_64, aarch64, musl)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64, aarch64)

[Unreleased]: https://github.com/thomasvincent/stonktop/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/thomasvincent/stonktop/releases/tag/v0.1.0
