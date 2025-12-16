//! Yahoo Finance API client for fetching stock and cryptocurrency quotes.
//!
//! Because checking your portfolio every 5 seconds is totally healthy behavior.

use crate::models::{MarketState, Quote, QuoteType};
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

/// The magical endpoint where dreams are made and destroyed.
const YAHOO_FINANCE_URL: &str = "https://query1.finance.yahoo.com/v7/finance/quote";

/// Pretending to be a real browser because Yahoo has trust issues.
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";

/// Yahoo Finance API client.
/// Your gateway to financial anxiety delivered in JSON format.
pub struct YahooFinanceClient {
    client: Client,
    timeout: Duration,
}

impl YahooFinanceClient {
    /// Create a new Yahoo Finance client.
    pub fn new(timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    /// Fetch quotes for multiple symbols.
    pub async fn get_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>> {
        if symbols.is_empty() {
            return Ok(Vec::new());
        }

        let symbols_param = symbols.join(",");
        let url = format!("{}?symbols={}", YAHOO_FINANCE_URL, symbols_param);

        let response = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .context("Failed to fetch quotes from Yahoo Finance")?;

        if !response.status().is_success() {
            anyhow::bail!("Yahoo Finance API returned error: {}", response.status());
        }

        let data: YahooResponse = response
            .json()
            .await
            .context("Failed to parse Yahoo Finance response")?;

        let quotes = data
            .quote_response
            .result
            .into_iter()
            .map(|r| r.into_quote())
            .collect();

        Ok(quotes)
    }

    /// Fetch a single quote.
    /// For when you only need to be disappointed by one stock at a time.
    #[allow(dead_code)] // Reserved for future regret-checking functionality
    pub async fn get_quote(&self, symbol: &str) -> Result<Quote> {
        let quotes = self.get_quotes(&[symbol.to_string()]).await?;
        quotes
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No quote found for symbol: {}", symbol))
    }
}

impl Default for YahooFinanceClient {
    fn default() -> Self {
        Self::new(10).expect("Failed to create default client")
    }
}

// Yahoo Finance API response structures

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YahooResponse {
    quote_response: QuoteResponse,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    result: Vec<YahooQuote>,
    #[allow(dead_code)]
    error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YahooQuote {
    symbol: String,
    #[serde(default)]
    short_name: Option<String>,
    #[serde(default)]
    long_name: Option<String>,
    #[serde(default)]
    regular_market_price: Option<f64>,
    #[serde(default)]
    regular_market_change: Option<f64>,
    #[serde(default)]
    regular_market_change_percent: Option<f64>,
    #[serde(default)]
    regular_market_previous_close: Option<f64>,
    #[serde(default)]
    regular_market_open: Option<f64>,
    #[serde(default)]
    regular_market_day_high: Option<f64>,
    #[serde(default)]
    regular_market_day_low: Option<f64>,
    #[serde(default)]
    fifty_two_week_high: Option<f64>,
    #[serde(default)]
    fifty_two_week_low: Option<f64>,
    #[serde(default)]
    regular_market_volume: Option<u64>,
    #[serde(default)]
    average_daily_volume3_month: Option<u64>,
    #[serde(default)]
    market_cap: Option<u64>,
    #[serde(default)]
    currency: Option<String>,
    #[serde(default)]
    exchange: Option<String>,
    #[serde(default)]
    quote_type: Option<String>,
    #[serde(default)]
    market_state: Option<String>,
    #[serde(default)]
    regular_market_time: Option<i64>,
}

impl YahooQuote {
    fn into_quote(self) -> Quote {
        Quote {
            symbol: self.symbol,
            name: self
                .short_name
                .or(self.long_name)
                .unwrap_or_else(|| "Unknown".to_string()),
            price: self.regular_market_price.unwrap_or(0.0),
            change: self.regular_market_change.unwrap_or(0.0),
            change_percent: self.regular_market_change_percent.unwrap_or(0.0),
            previous_close: self.regular_market_previous_close.unwrap_or(0.0),
            open: self.regular_market_open.unwrap_or(0.0),
            day_high: self.regular_market_day_high.unwrap_or(0.0),
            day_low: self.regular_market_day_low.unwrap_or(0.0),
            year_high: self.fifty_two_week_high.unwrap_or(0.0),
            year_low: self.fifty_two_week_low.unwrap_or(0.0),
            volume: self.regular_market_volume.unwrap_or(0),
            avg_volume: self.average_daily_volume3_month.unwrap_or(0),
            market_cap: self.market_cap,
            currency: self.currency.unwrap_or_else(|| "USD".to_string()),
            exchange: self.exchange.unwrap_or_default(),
            quote_type: parse_quote_type(self.quote_type.as_deref()),
            market_state: parse_market_state(self.market_state.as_deref()),
            timestamp: self
                .regular_market_time
                .map(|t| Utc.timestamp_opt(t, 0).unwrap())
                .unwrap_or_else(Utc::now),
        }
    }
}

fn parse_quote_type(s: Option<&str>) -> QuoteType {
    match s {
        Some("EQUITY") => QuoteType::Equity,
        Some("CRYPTOCURRENCY") => QuoteType::Cryptocurrency,
        Some("ETF") => QuoteType::Etf,
        Some("MUTUALFUND") => QuoteType::MutualFund,
        Some("INDEX") => QuoteType::Index,
        Some("CURRENCY") => QuoteType::Currency,
        Some("FUTURE") => QuoteType::Future,
        Some("OPTION") => QuoteType::Option,
        _ => QuoteType::Equity,
    }
}

fn parse_market_state(s: Option<&str>) -> MarketState {
    match s {
        Some("PRE") => MarketState::Pre,
        Some("REGULAR") => MarketState::Regular,
        Some("POST") | Some("POSTPOST") => MarketState::Post,
        _ => MarketState::Closed,
    }
}

/// Symbol shortcuts for common cryptocurrencies.
/// Because typing "-USD" is too much work for crypto bros.
pub fn expand_symbol(symbol: &str) -> String {
    // Handle shorthand crypto symbols like "BTC.X" -> "BTC-USD"
    // The .X suffix is like X marks the spot, but for losing money
    if let Some(base) = symbol.strip_suffix(".X") {
        return format!("{}-USD", base);
    }

    // Common crypto shortcuts
    let shortcuts: HashMap<&str, &str> = [
        ("BTC", "BTC-USD"),
        ("ETH", "ETH-USD"),
        ("SOL", "SOL-USD"),
        ("DOGE", "DOGE-USD"),
        ("XRP", "XRP-USD"),
        ("ADA", "ADA-USD"),
        ("DOT", "DOT-USD"),
        ("MATIC", "MATIC-USD"),
        ("LINK", "LINK-USD"),
        ("UNI", "UNI-USD"),
        ("AVAX", "AVAX-USD"),
        ("ATOM", "ATOM-USD"),
        ("LTC", "LTC-USD"),
    ]
    .into_iter()
    .collect();

    // Only expand if it looks like a crypto symbol (all caps, short)
    if symbol.len() <= 5 && symbol.chars().all(|c| c.is_ascii_uppercase()) {
        if let Some(expanded) = shortcuts.get(symbol) {
            return expanded.to_string();
        }
    }

    symbol.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_symbol_crypto_shorthand() {
        assert_eq!(expand_symbol("BTC.X"), "BTC-USD");
        assert_eq!(expand_symbol("ETH.X"), "ETH-USD");
    }

    #[test]
    fn test_expand_symbol_crypto_shortcuts() {
        assert_eq!(expand_symbol("BTC"), "BTC-USD");
        assert_eq!(expand_symbol("ETH"), "ETH-USD");
    }

    #[test]
    fn test_expand_symbol_stock() {
        assert_eq!(expand_symbol("AAPL"), "AAPL");
        assert_eq!(expand_symbol("GOOGL"), "GOOGL");
    }
}
