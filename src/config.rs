//! Configuration file handling with TOML support.
//!
//! Because hardcoding your portfolio would be too easy.

use crate::models::Holding;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Application configuration loaded from TOML file.
/// Where you define which assets will keep you up at night.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// General settings
    #[serde(default)]
    pub general: GeneralConfig,

    /// Watchlist symbols
    #[serde(default)]
    pub watchlist: WatchlistConfig,

    /// Holdings/portfolio configuration
    #[serde(default)]
    pub holdings: Vec<HoldingConfig>,

    /// Display settings
    #[serde(default)]
    pub display: DisplayConfig,

    /// Color scheme
    #[serde(default)]
    pub colors: ColorConfig,

    /// Groups of symbols
    #[serde(default)]
    pub groups: HashMap<String, Vec<String>>,
}

/// General application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Refresh interval in seconds
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval: f64,

    /// API timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Default currency for display
    #[serde(default = "default_currency")]
    pub currency: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            refresh_interval: default_refresh_interval(),
            timeout: default_timeout(),
            currency: default_currency(),
        }
    }
}

fn default_refresh_interval() -> f64 {
    5.0
}
fn default_timeout() -> u64 {
    10
}
fn default_currency() -> String {
    "USD".to_string()
}

/// Watchlist configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistConfig {
    /// List of symbols to watch
    #[serde(default)]
    pub symbols: Vec<String>,
}

/// Single holding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingConfig {
    /// Ticker symbol
    pub symbol: String,
    /// Number of shares/units
    pub quantity: f64,
    /// Cost basis per share
    pub cost_basis: f64,
}

impl From<HoldingConfig> for Holding {
    fn from(config: HoldingConfig) -> Self {
        Holding {
            symbol: config.symbol,
            quantity: config.quantity,
            cost_basis: config.cost_basis,
        }
    }
}

/// Display settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Show summary header
    #[serde(default = "default_true")]
    pub show_header: bool,

    /// Show fundamentals (open, high, low, etc.)
    #[serde(default)]
    pub show_fundamentals: bool,

    /// Show holdings view
    #[serde(default)]
    pub show_holdings: bool,

    /// Show separators between groups
    #[serde(default = "default_true")]
    pub show_separators: bool,

    /// Default sort field
    #[serde(default)]
    pub sort_by: String,

    /// Sort in descending order
    #[serde(default = "default_true")]
    pub sort_descending: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_header: true,
            show_fundamentals: false,
            show_holdings: false,
            show_separators: true,
            sort_by: "change_percent".to_string(),
            sort_descending: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Color configuration using hex codes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    /// Color for positive changes
    #[serde(default = "default_gain_color")]
    pub gain: String,

    /// Color for negative changes
    #[serde(default = "default_loss_color")]
    pub loss: String,

    /// Color for neutral/unchanged
    #[serde(default = "default_neutral_color")]
    pub neutral: String,

    /// Header background color
    #[serde(default = "default_header_color")]
    pub header: String,

    /// Border color
    #[serde(default = "default_border_color")]
    pub border: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            gain: default_gain_color(),
            loss: default_loss_color(),
            neutral: default_neutral_color(),
            header: default_header_color(),
            border: default_border_color(),
        }
    }
}

fn default_gain_color() -> String {
    "#00ff00".to_string()
}
fn default_loss_color() -> String {
    "#ff0000".to_string()
}
fn default_neutral_color() -> String {
    "#ffffff".to_string()
}
fn default_header_color() -> String {
    "#1e90ff".to_string()
}
fn default_border_color() -> String {
    "#444444".to_string()
}

impl Config {
    /// Load configuration from file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Load configuration from default location or create default.
    pub fn load_or_default() -> Self {
        if let Some(path) = Self::default_config_path() {
            if path.exists() {
                match Self::load(&path) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Warning: Failed to load config: {}", e);
                    }
                }
            }
        }
        Config::default()
    }

    /// Get the default configuration file path.
    pub fn default_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("stonktop").join("config.toml"))
    }

    /// Save configuration to file.
    /// For when you finally decide to commit to your investment strategy.
    #[allow(dead_code)] // Config export feature - because backup plans are underrated
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get all symbols from watchlist and holdings.
    pub fn all_symbols(&self) -> Vec<String> {
        let mut symbols: Vec<String> = self.watchlist.symbols.clone();

        // Add holding symbols
        for holding in &self.holdings {
            if !symbols.contains(&holding.symbol) {
                symbols.push(holding.symbol.clone());
            }
        }

        // Add group symbols
        for group_symbols in self.groups.values() {
            for symbol in group_symbols {
                if !symbols.contains(symbol) {
                    symbols.push(symbol.clone());
                }
            }
        }

        symbols
    }

    /// Get holdings as Holding structs.
    pub fn get_holdings(&self) -> Vec<Holding> {
        self.holdings.iter().cloned().map(Into::into).collect()
    }
}

/// Generate a sample configuration file content.
pub fn sample_config() -> &'static str {
    r##"# Stonktop Configuration File
# A top-like terminal UI for stock and crypto prices

[general]
# Refresh interval in seconds
refresh_interval = 5.0
# API timeout in seconds
timeout = 10
# Default currency for display
currency = "USD"

[watchlist]
# Symbols to track
symbols = [
    "AAPL",
    "GOOGL",
    "MSFT",
    "AMZN",
    "NVDA",
    "BTC-USD",
    "ETH-USD",
]

# Portfolio holdings (optional)
[[holdings]]
symbol = "AAPL"
quantity = 10
cost_basis = 150.00

[[holdings]]
symbol = "BTC-USD"
quantity = 0.5
cost_basis = 30000.00

[display]
# Show summary header
show_header = true
# Show fundamental data (open, high, low)
show_fundamentals = false
# Show portfolio holdings
show_holdings = false
# Show separators between groups
show_separators = true
# Default sort field: symbol, name, price, change, change_percent, volume, market_cap
sort_by = "change_percent"
# Sort in descending order
sort_descending = true

[colors]
# Colors in hex format
gain = "#00ff00"
loss = "#ff0000"
neutral = "#ffffff"
header = "#1e90ff"
border = "#444444"

# Symbol groups (for organizing watchlists)
[groups]
tech = ["AAPL", "GOOGL", "MSFT", "NVDA"]
crypto = ["BTC-USD", "ETH-USD", "SOL-USD"]
"##
}
