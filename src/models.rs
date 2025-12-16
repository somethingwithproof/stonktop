//! Data models for stock and cryptocurrency quotes.
//!
//! Structs that hold the numbers you'll obsessively refresh.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a financial quote for a stock or cryptocurrency.
/// Contains all the numbers you need to feel emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Ticker symbol (e.g., "AAPL", "BTC-USD")
    pub symbol: String,
    /// Full name of the security
    pub name: String,
    /// Current price
    pub price: f64,
    /// Price change from previous close
    pub change: f64,
    /// Percentage change from previous close
    pub change_percent: f64,
    /// Previous closing price
    pub previous_close: f64,
    /// Opening price for the day
    pub open: f64,
    /// Day's high price
    pub day_high: f64,
    /// Day's low price
    pub day_low: f64,
    /// 52-week high
    pub year_high: f64,
    /// 52-week low
    pub year_low: f64,
    /// Trading volume
    pub volume: u64,
    /// Average volume
    pub avg_volume: u64,
    /// Market capitalization
    pub market_cap: Option<u64>,
    /// Currency of the quote
    pub currency: String,
    /// Exchange where the security is traded
    pub exchange: String,
    /// Quote type (EQUITY, CRYPTOCURRENCY, ETF, etc.)
    pub quote_type: QuoteType,
    /// Market state (PRE, REGULAR, POST, CLOSED)
    pub market_state: MarketState,
    /// Timestamp of the quote
    pub timestamp: DateTime<Utc>,
}

impl Default for Quote {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            name: String::new(),
            price: 0.0,
            change: 0.0,
            change_percent: 0.0,
            previous_close: 0.0,
            open: 0.0,
            day_high: 0.0,
            day_low: 0.0,
            year_high: 0.0,
            year_low: 0.0,
            volume: 0,
            avg_volume: 0,
            market_cap: None,
            currency: "USD".to_string(),
            exchange: String::new(),
            quote_type: QuoteType::Equity,
            market_state: MarketState::Closed,
            timestamp: Utc::now(),
        }
    }
}

/// Type of financial instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QuoteType {
    #[default]
    Equity,
    Cryptocurrency,
    Etf,
    MutualFund,
    Index,
    Currency,
    Future,
    Option,
}

impl std::fmt::Display for QuoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteType::Equity => write!(f, "Stock"),
            QuoteType::Cryptocurrency => write!(f, "Crypto"),
            QuoteType::Etf => write!(f, "ETF"),
            QuoteType::MutualFund => write!(f, "Fund"),
            QuoteType::Index => write!(f, "Index"),
            QuoteType::Currency => write!(f, "Forex"),
            QuoteType::Future => write!(f, "Future"),
            QuoteType::Option => write!(f, "Option"),
        }
    }
}

/// Market trading state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MarketState {
    Pre,
    Regular,
    Post,
    #[default]
    Closed,
}

impl std::fmt::Display for MarketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketState::Pre => write!(f, "Pre"),
            MarketState::Regular => write!(f, "Open"),
            MarketState::Post => write!(f, "Post"),
            MarketState::Closed => write!(f, "Closed"),
        }
    }
}

/// Holding represents a position in a security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    /// Ticker symbol
    pub symbol: String,
    /// Number of shares/units held
    pub quantity: f64,
    /// Average cost basis per share
    pub cost_basis: f64,
}

impl Holding {
    /// Calculate total cost of the holding.
    pub fn total_cost(&self) -> f64 {
        self.quantity * self.cost_basis
    }

    /// Calculate current value given current price.
    pub fn current_value(&self, price: f64) -> f64 {
        self.quantity * price
    }

    /// Calculate profit/loss given current price.
    pub fn profit_loss(&self, price: f64) -> f64 {
        self.current_value(price) - self.total_cost()
    }

    /// Calculate profit/loss percentage given current price.
    pub fn profit_loss_percent(&self, price: f64) -> f64 {
        if self.total_cost() == 0.0 {
            0.0
        } else {
            (self.profit_loss(price) / self.total_cost()) * 100.0
        }
    }
}

/// Sort order for displaying quotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SortOrder {
    #[default]
    Symbol,
    Name,
    Price,
    Change,
    ChangePercent,
    Volume,
    MarketCap,
}

impl SortOrder {
    /// Get the next sort order in cycle.
    pub fn next(self) -> Self {
        match self {
            SortOrder::Symbol => SortOrder::Name,
            SortOrder::Name => SortOrder::Price,
            SortOrder::Price => SortOrder::Change,
            SortOrder::Change => SortOrder::ChangePercent,
            SortOrder::ChangePercent => SortOrder::Volume,
            SortOrder::Volume => SortOrder::MarketCap,
            SortOrder::MarketCap => SortOrder::Symbol,
        }
    }

    /// Get column header name.
    pub fn header(&self) -> &'static str {
        match self {
            SortOrder::Symbol => "SYMBOL",
            SortOrder::Name => "NAME",
            SortOrder::Price => "PRICE",
            SortOrder::Change => "CHANGE",
            SortOrder::ChangePercent => "CHG%",
            SortOrder::Volume => "VOLUME",
            SortOrder::MarketCap => "MKT CAP",
        }
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn toggle(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
}
