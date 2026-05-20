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

/// Port: abstraction for fetching quotes from any data source.
/// Implement this trait to plug in Yahoo Finance, Alpaca, Coinbase, mocks, etc.
#[async_trait::async_trait]
pub trait QuoteProvider: Send + Sync {
    async fn get_quotes(&self, symbols: &[String]) -> anyhow::Result<Vec<Quote>>;
}

/// Rolling buffer of recent prices for sparkline rendering.
#[derive(Debug, Clone, Default)]
pub struct PriceHistory {
    prices: Vec<f64>,
    max_len: usize,
}

impl PriceHistory {
    pub fn new(max_len: usize) -> Self {
        Self {
            prices: Vec::with_capacity(max_len),
            max_len,
        }
    }

    pub fn push(&mut self, price: f64) {
        if self.prices.len() >= self.max_len {
            self.prices.remove(0);
        }
        self.prices.push(price);
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.prices
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.prices.is_empty()
    }

    pub fn to_sparkline_data(&self) -> Vec<u64> {
        if self.prices.is_empty() {
            return Vec::new();
        }
        let min = self.prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self
            .prices
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;
        if range == 0.0 {
            return vec![4; self.prices.len()];
        }
        self.prices
            .iter()
            .map(|&p| ((p - min) / range * 7.0).round() as u64)
            .collect()
    }
}

/// Alert threshold for a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub symbol: String,
    pub above: Option<f64>,
    pub below: Option<f64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- PriceHistory tests ---

    #[test]
    fn test_price_history_new() {
        let h = PriceHistory::new(5);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert_eq!(h.as_slice(), &[] as &[f64]);
    }

    #[test]
    fn test_price_history_push() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(101.0);
        assert_eq!(h.len(), 2);
        assert_eq!(h.as_slice(), &[100.0, 101.0]);
    }

    #[test]
    fn test_price_history_rolling_eviction() {
        let mut h = PriceHistory::new(3);
        h.push(1.0);
        h.push(2.0);
        h.push(3.0);
        assert_eq!(h.len(), 3);
        h.push(4.0);
        assert_eq!(h.len(), 3);
        assert_eq!(h.as_slice(), &[2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_price_history_sparkline_empty() {
        let h = PriceHistory::new(5);
        assert!(h.to_sparkline_data().is_empty());
    }

    #[test]
    fn test_price_history_sparkline_flat() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(100.0);
        h.push(100.0);
        let data = h.to_sparkline_data();
        assert_eq!(data, vec![4, 4, 4]);
    }

    #[test]
    fn test_price_history_sparkline_ascending() {
        let mut h = PriceHistory::new(5);
        h.push(0.0);
        h.push(3.5);
        h.push(7.0);
        let data = h.to_sparkline_data();
        assert_eq!(data[0], 0); // min
        assert_eq!(data[2], 7); // max
    }

    #[test]
    fn test_price_history_sparkline_descending() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(50.0);
        h.push(0.0);
        let data = h.to_sparkline_data();
        assert_eq!(data[0], 7); // max (100)
        assert_eq!(data[2], 0); // min (0)
    }

    #[test]
    fn test_price_history_single_value() {
        let mut h = PriceHistory::new(5);
        h.push(42.0);
        let data = h.to_sparkline_data();
        assert_eq!(data, vec![4]); // flat = midpoint
    }

    // --- SortDirection tests ---

    #[test]
    fn test_sort_direction_toggle() {
        assert_eq!(SortDirection::Ascending.toggle(), SortDirection::Descending);
        assert_eq!(SortDirection::Descending.toggle(), SortDirection::Ascending);
    }

    // --- SortOrder tests ---

    #[test]
    fn test_sort_order_next_cycles() {
        let mut order = SortOrder::Symbol;
        for _ in 0..7 {
            order = order.next();
        }
        assert_eq!(order, SortOrder::Symbol); // full cycle
    }

    #[test]
    fn test_sort_order_header() {
        assert_eq!(SortOrder::Symbol.header(), "SYMBOL");
        assert_eq!(SortOrder::Price.header(), "PRICE");
        assert_eq!(SortOrder::MarketCap.header(), "MKT CAP");
    }

    // --- QuoteType Display tests ---

    #[test]
    fn test_quote_type_display() {
        assert_eq!(format!("{}", QuoteType::Equity), "Stock");
        assert_eq!(format!("{}", QuoteType::Cryptocurrency), "Crypto");
        assert_eq!(format!("{}", QuoteType::Etf), "ETF");
        assert_eq!(format!("{}", QuoteType::Currency), "Forex");
    }

    // --- MarketState Display tests ---

    #[test]
    fn test_market_state_display() {
        assert_eq!(format!("{}", MarketState::Regular), "Open");
        assert_eq!(format!("{}", MarketState::Closed), "Closed");
        assert_eq!(format!("{}", MarketState::Pre), "Pre");
        assert_eq!(format!("{}", MarketState::Post), "Post");
    }

    // --- Holding tests ---

    #[test]
    fn test_holding_total_cost() {
        let h = Holding {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            cost_basis: 150.0,
        };
        assert!((h.total_cost() - 1500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_holding_profit_loss() {
        let h = Holding {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            cost_basis: 150.0,
        };
        assert!((h.profit_loss(200.0) - 500.0).abs() < f64::EPSILON);
        assert!((h.profit_loss(100.0) - (-500.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_holding_profit_loss_percent() {
        let h = Holding {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            cost_basis: 100.0,
        };
        assert!((h.profit_loss_percent(200.0) - 100.0).abs() < f64::EPSILON);
    }
}
