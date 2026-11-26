// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Command-line interface with top-like options.
//!
//! All the flags you need to customize your financial anxiety experience.

use crate::i18n::Locale;
use clap::{CommandFactory, FromArgMatches, Parser, ValueEnum};
use std::path::PathBuf;

fn parse_positive_f64(s: &str) -> Result<f64, String> {
    let val: f64 = s.parse().map_err(|e| format!("{e}"))?;
    if !val.is_finite() || val <= 0.0 {
        return Err(Locale::detect().catalog().err_delay_positive.to_string());
    }
    Ok(val)
}

/// A top-like terminal UI for monitoring stock and cryptocurrency prices.
/// Like htop, but for watching numbers go down instead of CPU usage.
///
/// Stonktop provides real-time quotes for stocks and cryptocurrencies
/// with a familiar top-like interface. It supports watchlists, portfolio
/// tracking, and multiple display modes.
#[derive(Parser, Debug, Clone)]
#[command(name = "stonktop")]
#[command(author = "Thomas Vincent")]
#[command(version)]
#[command(about = "A top-like terminal UI for stock and crypto prices", long_about = None)]
pub struct Args {
    /// Symbols to watch (comma-separated)
    ///
    /// Examples: AAPL,GOOGL,MSFT or BTC-USD,ETH-USD
    /// Shortcuts: BTC.X expands to BTC-USD
    #[arg(short = 's', long, value_delimiter = ',', env = "STONKTOP_SYMBOLS")]
    pub symbols: Option<Vec<String>>,

    /// Refresh delay in seconds (like top -d)
    #[arg(short = 'd', long, default_value = "5", env = "STONKTOP_DELAY", value_parser = parse_positive_f64)]
    pub delay: f64,

    /// Number of iterations before exiting (like top -n)
    ///
    /// 0 means infinite
    #[arg(short = 'n', long, default_value = "0")]
    pub iterations: u64,

    /// Batch mode - useful for sending output to another program (like top -b)
    #[arg(short = 'b', long)]
    pub batch: bool,

    /// Secure mode - disables interactive commands
    #[arg(short = 'S', long)]
    pub secure: bool,

    /// Configuration file path
    #[arg(short = 'c', long, env = "STONKTOP_CONFIG")]
    pub config: Option<PathBuf>,

    /// Initial sort field (like top -o)
    #[arg(short = 'o', long, value_enum, default_value = "change-percent")]
    pub sort: SortField,

    /// Reverse sort order
    #[arg(short = 'r', long)]
    pub reverse: bool,

    /// Show holdings/portfolio view
    #[arg(short = 'H', long)]
    pub holdings: bool,

    /// Currency for display (ISO 4217 code)
    #[arg(long, default_value = "USD")]
    pub currency: String,

    /// Enable color output (auto, always, never)
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorMode,

    /// Verbose output - show more details
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// API timeout in seconds
    #[arg(long, default_value = "10")]
    pub timeout: u64,

    /// Output format for batch mode (table, json, csv)
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Generate a sample configuration file
    #[arg(long)]
    pub init: bool,

    /// Force overwrite existing config file (use with --init)
    #[arg(long)]
    pub force: bool,

    /// Display language (e.g. en, de, ja, zh-CN, pt-BR)
    #[arg(long, env = "STONKTOP_LANG")]
    pub lang: Option<String>,
}

/// Sort field options (similar to top's sort fields).
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum SortField {
    /// Sort by symbol name
    Symbol,
    /// Sort by company/asset name
    Name,
    /// Sort by current price
    Price,
    /// Sort by price change
    Change,
    /// Sort by percentage change (default)
    #[default]
    ChangePercent,
    /// Sort by trading volume
    Volume,
    /// Sort by market capitalization
    MarketCap,
}

impl From<SortField> for crate::models::SortOrder {
    fn from(field: SortField) -> Self {
        match field {
            SortField::Symbol => crate::models::SortOrder::Symbol,
            SortField::Name => crate::models::SortOrder::Name,
            SortField::Price => crate::models::SortOrder::Price,
            SortField::Change => crate::models::SortOrder::Change,
            SortField::ChangePercent => crate::models::SortOrder::ChangePercent,
            SortField::Volume => crate::models::SortOrder::Volume,
            SortField::MarketCap => crate::models::SortOrder::MarketCap,
        }
    }
}

/// Color output mode.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ColorMode {
    /// Automatically detect terminal capabilities
    #[default]
    Auto,
    /// Always use colors
    Always,
    /// Never use colors
    Never,
}

/// Output format for batch mode.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    /// Classic table layout (default)
    #[default]
    Table,
    /// JSON output for programmatic consumption
    Json,
    /// CSV output for spreadsheet warriors
    Csv,
}

impl Args {
    /// Build a clap command with localized help text for the active locale.
    #[allow(dead_code)]
    pub fn localized_command(locale: Locale) -> clap::Command {
        let cat = locale.catalog();
        Args::command()
            .about(cat.cli_about)
            .mut_arg("symbols", |a| a.help(cat.cli_opt_symbols))
            .mut_arg("delay", |a| a.help(cat.cli_opt_delay))
            .mut_arg("iterations", |a| a.help(cat.cli_opt_iterations))
            .mut_arg("batch", |a| a.help(cat.cli_opt_batch))
            .mut_arg("secure", |a| a.help(cat.cli_opt_secure))
            .mut_arg("config", |a| a.help(cat.cli_opt_config))
            .mut_arg("sort", |a| a.help(cat.cli_opt_sort))
            .mut_arg("reverse", |a| a.help(cat.cli_opt_reverse))
            .mut_arg("holdings", |a| a.help(cat.cli_opt_holdings))
            .mut_arg("currency", |a| a.help(cat.cli_opt_currency))
            .mut_arg("color", |a| a.help(cat.cli_opt_color))
            .mut_arg("verbose", |a| a.help(cat.cli_opt_verbose))
            .mut_arg("timeout", |a| a.help(cat.cli_opt_timeout))
            .mut_arg("format", |a| a.help(cat.cli_opt_format))
            .mut_arg("init", |a| a.help(cat.cli_opt_init))
            .mut_arg("force", |a| a.help(cat.cli_opt_force))
            .mut_arg("lang", |a| a.help(cat.cli_opt_lang))
    }

    /// Parse command line arguments with localized help text.
    /// Where your journey to financial enlightenment begins.
    #[allow(dead_code)]
    pub fn parse_args() -> Self {
        let raw_args: Vec<String> = std::env::args().collect();
        let locale = Locale::resolve(crate::i18n::extract_cli_lang(&raw_args).as_deref());
        let matches = Self::localized_command(locale).get_matches_from(raw_args);
        match Self::from_arg_matches(&matches) {
            Ok(args) => args,
            Err(err) => err.exit(),
        }
    }

    // use_colors and atty_check were removed; color mode is wired via App.ui.color_mode and UiColors::for_app.
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn test_default_args() {
        let args = Args::parse_from(["stonktop"]);
        assert_eq!(args.delay, 5.0);
        assert_eq!(args.iterations, 0);
        assert!(!args.batch);
    }

    #[test]
    fn test_symbols_parsing() {
        let args = Args::parse_from(["stonktop", "-s", "AAPL,GOOGL,MSFT"]);
        assert_eq!(
            args.symbols,
            Some(vec![
                "AAPL".to_string(),
                "GOOGL".to_string(),
                "MSFT".to_string()
            ])
        );
    }

    #[test]
    fn test_delay_and_iterations() {
        let args = Args::parse_from(["stonktop", "-d", "2.5", "-n", "10"]);
        assert_eq!(args.delay, 2.5);
        assert_eq!(args.iterations, 10);
    }
}
