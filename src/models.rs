// SPDX-FileCopyrightText: 2025-2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Data models for stock and cryptocurrency quotes.
//!
//! Structs that hold the numbers you'll obsessively refresh.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Price value (f64 alias for display/quote values from Yahoo API).
/// See review for rationale: sufficient for TUI, future decimal possible.
pub type Price = f64;

/// Represents a financial quote for a stock or cryptocurrency.
/// Contains all the numbers you need to feel emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Ticker symbol (e.g., "AAPL", "BTC-USD")
    pub symbol: String,
    /// Full name of the security
    pub name: String,
    /// Current price
    pub price: Price,
    /// Price change from previous close
    pub change: Price,
    /// Percentage change from previous close
    pub change_percent: f64,
    /// Previous closing price
    pub previous_close: Price,
    /// Opening price for the day
    pub open: Price,
    /// Day's high price
    pub day_high: Price,
    /// Day's low price
    pub day_low: Price,
    /// 52-week high
    pub year_high: Price,
    /// 52-week low
    pub year_low: Price,
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

impl QuoteType {
    /// Return the localized label for this quote type.
    #[allow(dead_code)]
    pub const fn localized(self, cat: &crate::i18n::Catalog) -> &'static str {
        match self {
            QuoteType::Equity => cat.quote_type_stock,
            QuoteType::Cryptocurrency => cat.quote_type_crypto,
            QuoteType::Etf => cat.quote_type_etf,
            QuoteType::MutualFund => cat.quote_type_fund,
            QuoteType::Index => cat.quote_type_index,
            QuoteType::Currency => cat.quote_type_forex,
            QuoteType::Future => cat.quote_type_future,
            QuoteType::Option => cat.quote_type_option,
        }
    }
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

impl MarketState {
    /// Return the localized label for this market state.
    #[allow(dead_code)]
    pub const fn localized(self, cat: &crate::i18n::Catalog) -> &'static str {
        match self {
            MarketState::Pre => cat.market_state_pre,
            MarketState::Regular => cat.market_state_open,
            MarketState::Post => cat.market_state_post,
            MarketState::Closed => cat.market_state_closed,
        }
    }
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
    pub fn current_value(&self, price: Price) -> f64 {
        self.quantity * price
    }

    /// Calculate profit/loss given current price.
    pub fn profit_loss(&self, price: Price) -> f64 {
        self.current_value(price) - self.total_cost()
    }

    /// Calculate profit/loss percentage given current price.
    pub fn profit_loss_percent(&self, price: Price) -> f64 {
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
    #[allow(dead_code)]
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

    /// Get localized column header name.
    #[allow(dead_code)]
    pub const fn localized_header(self, cat: &crate::i18n::Catalog) -> &'static str {
        match self {
            SortOrder::Symbol => cat.col_symbol,
            SortOrder::Name => cat.col_name,
            SortOrder::Price => cat.col_price,
            SortOrder::Change => cat.col_change,
            SortOrder::ChangePercent => cat.col_chg_pct,
            SortOrder::Volume => cat.col_volume,
            SortOrder::MarketCap => cat.col_mkt_cap,
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
///
/// Uses a VecDeque for O(1) front eviction and eagerly caches sparkline data
/// and min/max on each push so rendering is allocation-free.
#[derive(Debug, Clone)]
pub struct PriceHistory {
    prices: VecDeque<Price>,
    max_len: usize,
    sparkline_cache: Vec<u64>,
    min_max_cache: Option<(Price, Price)>,
}

impl Default for PriceHistory {
    fn default() -> Self {
        Self {
            prices: VecDeque::new(),
            max_len: 1,
            sparkline_cache: Vec::new(),
            min_max_cache: None,
        }
    }
}

impl PriceHistory {
    pub fn new(max_len: usize) -> Self {
        assert!(max_len > 0, "PriceHistory max_len must be at least 1");
        Self {
            prices: VecDeque::with_capacity(max_len),
            max_len,
            sparkline_cache: Vec::new(),
            min_max_cache: None,
        }
    }

    pub fn push(&mut self, price: Price) {
        if !price.is_finite() {
            return;
        }
        if self.prices.len() >= self.max_len {
            self.prices.pop_front();
        }
        self.prices.push_back(price);
        self.recompute_cache();
    }

    fn recompute_cache(&mut self) {
        if self.prices.is_empty() {
            self.sparkline_cache.clear();
            self.min_max_cache = None;
            return;
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &p in &self.prices {
            if p < min {
                min = p;
            }
            if p > max {
                max = p;
            }
        }
        self.min_max_cache = Some((min, max));

        let range = max - min;
        if range == 0.0 {
            self.sparkline_cache = vec![4; self.prices.len()];
        } else {
            self.sparkline_cache = self
                .prices
                .iter()
                .map(|&p| ((p - min) / range * 7.0).round() as u64)
                .collect();
        }
    }

    pub fn sparkline_data(&self) -> &[u64] {
        &self.sparkline_cache
    }

    pub fn min_max(&self) -> Option<(Price, Price)> {
        self.min_max_cache
    }

    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec<Price> {
        self.prices.iter().copied().collect()
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.prices.is_empty()
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
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    // --- PriceHistory tests ---

    #[test]
    fn test_price_history_new() {
        let h = PriceHistory::new(5);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert_eq!(h.to_vec(), vec![] as Vec<Price>);
    }

    #[test]
    fn test_price_history_push() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(101.0);
        assert_eq!(h.len(), 2);
        assert_eq!(h.to_vec(), vec![100.0, 101.0]);
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
        assert_eq!(h.to_vec(), vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_price_history_sparkline_empty() {
        let h = PriceHistory::new(5);
        assert_eq!(h.sparkline_data().len(), 0);
    }

    #[test]
    fn test_price_history_sparkline_flat() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(100.0);
        h.push(100.0);
        assert_eq!(h.sparkline_data(), &[4, 4, 4]);
    }

    #[test]
    fn test_price_history_sparkline_ascending() {
        let mut h = PriceHistory::new(5);
        h.push(0.0);
        h.push(3.5);
        h.push(7.0);
        let data = h.sparkline_data();
        assert_eq!(data[0], 0); // min
        assert_eq!(data[2], 7); // max
    }

    #[test]
    fn test_price_history_sparkline_descending() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(50.0);
        h.push(0.0);
        let data = h.sparkline_data();
        assert_eq!(data[0], 7); // max (100)
        assert_eq!(data[2], 0); // min (0)
    }

    #[test]
    fn test_price_history_single_value() {
        let mut h = PriceHistory::new(5);
        h.push(42.0);
        assert_eq!(h.sparkline_data(), &[4]); // flat = midpoint
    }

    #[test]
    #[should_panic(expected = "max_len must be at least 1")]
    fn test_price_history_zero_max_len_panics() {
        PriceHistory::new(0);
    }

    #[test]
    fn test_price_history_push_nan_ignored() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(f64::NAN);
        h.push(101.0);
        assert_eq!(h.len(), 2);
        assert_eq!(h.to_vec(), vec![100.0, 101.0]);
    }

    #[test]
    fn test_price_history_push_infinity_ignored() {
        let mut h = PriceHistory::new(5);
        h.push(100.0);
        h.push(f64::INFINITY);
        h.push(f64::NEG_INFINITY);
        h.push(101.0);
        assert_eq!(h.len(), 2);
        assert_eq!(h.to_vec(), vec![100.0, 101.0]);
    }

    #[test]
    fn test_price_history_min_max_empty() {
        let h = PriceHistory::new(5);
        assert!(h.min_max().is_none());
    }

    #[test]
    fn test_price_history_min_max() {
        let mut h = PriceHistory::new(5);
        h.push(50.0);
        h.push(100.0);
        h.push(75.0);
        let (min, max) = h.min_max().unwrap();
        assert!((min - 50.0).abs() < f64::EPSILON);
        assert!((max - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_price_history_sparkline_data_cached() {
        let mut h = PriceHistory::new(5);
        h.push(0.0);
        h.push(7.0);
        assert_eq!(h.sparkline_data(), &[0, 7]);
        assert_eq!(h.sparkline_data(), &[0, 7]);
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
        assert_eq!(SortOrder::Name.header(), "NAME");
        assert_eq!(SortOrder::Price.header(), "PRICE");
        assert_eq!(SortOrder::Change.header(), "CHANGE");
        assert_eq!(SortOrder::ChangePercent.header(), "CHG%");
        assert_eq!(SortOrder::Volume.header(), "VOLUME");
        assert_eq!(SortOrder::MarketCap.header(), "MKT CAP");
    }

    #[test]
    fn test_sort_order_localized_header() {
        use crate::i18n::Locale;
        let variants = [
            SortOrder::Symbol,
            SortOrder::Name,
            SortOrder::Price,
            SortOrder::Change,
            SortOrder::ChangePercent,
            SortOrder::Volume,
            SortOrder::MarketCap,
        ];
        for locale in [Locale::En, Locale::De, Locale::Ar] {
            let cat = locale.catalog();
            assert_eq!(SortOrder::Symbol.localized_header(cat), cat.col_symbol);
            assert_eq!(SortOrder::Name.localized_header(cat), cat.col_name);
            assert_eq!(SortOrder::Price.localized_header(cat), cat.col_price);
            assert_eq!(SortOrder::Change.localized_header(cat), cat.col_change);
            assert_eq!(
                SortOrder::ChangePercent.localized_header(cat),
                cat.col_chg_pct
            );
            assert_eq!(SortOrder::Volume.localized_header(cat), cat.col_volume);
            assert_eq!(SortOrder::MarketCap.localized_header(cat), cat.col_mkt_cap);
            for variant in variants {
                assert_ne!(variant.localized_header(cat), "");
            }
        }
    }

    // --- QuoteType Display & localized tests ---

    #[test]
    fn test_quote_type_display() {
        assert_eq!(format!("{}", QuoteType::Equity), "Stock");
        assert_eq!(format!("{}", QuoteType::Cryptocurrency), "Crypto");
        assert_eq!(format!("{}", QuoteType::Etf), "ETF");
        assert_eq!(format!("{}", QuoteType::MutualFund), "Fund");
        assert_eq!(format!("{}", QuoteType::Index), "Index");
        assert_eq!(format!("{}", QuoteType::Currency), "Forex");
        assert_eq!(format!("{}", QuoteType::Future), "Future");
        assert_eq!(format!("{}", QuoteType::Option), "Option");
    }

    #[test]
    fn test_quote_type_localized() {
        use crate::i18n::Locale;
        let variants = [
            QuoteType::Equity,
            QuoteType::Cryptocurrency,
            QuoteType::Etf,
            QuoteType::MutualFund,
            QuoteType::Index,
            QuoteType::Currency,
            QuoteType::Future,
            QuoteType::Option,
        ];
        for locale in [Locale::En, Locale::De, Locale::Ar] {
            let cat = locale.catalog();
            assert_eq!(QuoteType::Equity.localized(cat), cat.quote_type_stock);
            assert_eq!(
                QuoteType::Cryptocurrency.localized(cat),
                cat.quote_type_crypto
            );
            assert_eq!(QuoteType::Etf.localized(cat), cat.quote_type_etf);
            assert_eq!(QuoteType::MutualFund.localized(cat), cat.quote_type_fund);
            assert_eq!(QuoteType::Index.localized(cat), cat.quote_type_index);
            assert_eq!(QuoteType::Currency.localized(cat), cat.quote_type_forex);
            assert_eq!(QuoteType::Future.localized(cat), cat.quote_type_future);
            assert_eq!(QuoteType::Option.localized(cat), cat.quote_type_option);
            for variant in variants {
                assert_ne!(variant.localized(cat), "");
            }
        }
        assert_eq!(
            QuoteType::Equity.localized(Locale::En.catalog()),
            format!("{}", QuoteType::Equity)
        );
    }

    // --- MarketState Display & localized tests ---

    #[test]
    fn test_market_state_display() {
        assert_eq!(format!("{}", MarketState::Regular), "Open");
        assert_eq!(format!("{}", MarketState::Closed), "Closed");
        assert_eq!(format!("{}", MarketState::Pre), "Pre");
        assert_eq!(format!("{}", MarketState::Post), "Post");
    }

    #[test]
    fn test_market_state_localized() {
        use crate::i18n::Locale;
        let variants = [
            MarketState::Pre,
            MarketState::Regular,
            MarketState::Post,
            MarketState::Closed,
        ];
        for locale in [Locale::En, Locale::De, Locale::Ar] {
            let cat = locale.catalog();
            assert_eq!(MarketState::Pre.localized(cat), cat.market_state_pre);
            assert_eq!(MarketState::Regular.localized(cat), cat.market_state_open);
            assert_eq!(MarketState::Post.localized(cat), cat.market_state_post);
            assert_eq!(MarketState::Closed.localized(cat), cat.market_state_closed);
            for variant in variants {
                assert_ne!(variant.localized(cat), "");
            }
        }
        assert_eq!(
            MarketState::Regular.localized(Locale::En.catalog()),
            format!("{}", MarketState::Regular)
        );
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
