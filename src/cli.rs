//! Command-line interface with top-like options.
//!
//! All the flags you need to customize your financial anxiety experience.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
    #[arg(short = 'd', long, default_value = "5", env = "STONKTOP_DELAY")]
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

    /// Show only top N symbols
    #[arg(short = 't', long)]
    pub top: Option<usize>,

    /// Filter by quote type
    #[arg(short = 'f', long, value_enum)]
    pub filter: Option<FilterType>,

    /// Hide summary header
    #[arg(long)]
    pub no_header: bool,

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

/// Filter options for quote types.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FilterType {
    /// Show only stocks
    Stocks,
    /// Show only cryptocurrencies
    Crypto,
    /// Show only ETFs
    Etf,
    /// Show only indices
    Index,
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

impl Args {
    /// Parse command line arguments.
    /// Where your journey to financial enlightenment begins.
    pub fn parse_args() -> Self {
        Args::parse()
    }

    /// Check if colors should be enabled.
    /// Because red and green are the only colors that matter in finance.
    #[allow(dead_code)] // Reserved for when we implement --no-feelings mode
    pub fn use_colors(&self) -> bool {
        match self.color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => {
                // Check if stdout is a terminal
                atty_check()
            }
        }
    }
}

/// Check if stdout is a terminal.
/// Spoiler: it probably is, unless you're piping your tears to /dev/null.
#[allow(dead_code)] // Used by use_colors which is reserved for future features
fn atty_check() -> bool {
    // Simple check - in production you might use the `atty` crate
    std::env::var("TERM").is_ok()
}

#[cfg(test)]
mod tests {
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
