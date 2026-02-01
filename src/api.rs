//! Yahoo Finance API client for fetching stock and cryptocurrency quotes.
//!
//! Because checking your portfolio every 5 seconds is totally healthy behavior.

use crate::models::{MarketState, Quote, QuoteType};
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

/// The v8 chart API endpoint - the one that still works (for now).
const YAHOO_CHART_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";

/// Pretending to be a real browser because Yahoo has trust issues.
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

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

    /// Fetch quotes for multiple symbols using parallel requests.
    /// Yahoo's v8 chart API only supports one symbol at a time, so we parallelize.
    pub async fn get_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>> {
        if symbols.is_empty() {
            return Ok(Vec::new());
        }

        // Fetch all symbols in parallel
        let futures: Vec<_> = symbols
            .iter()
            .map(|symbol| self.fetch_single_quote(symbol))
            .collect();

        let results = join_all(futures).await;

        // Collect successful results, skip failures
        let quotes: Vec<Quote> = results.into_iter().filter_map(|r| r.ok()).collect();

        Ok(quotes)
    }

    /// Fetch a single quote from the v8 chart API.
    async fn fetch_single_quote(&self, symbol: &str) -> Result<Quote> {
        // Symbol goes in the path, not as a query parameter
        let url = format!("{}/{}?interval=1d&range=1d", YAHOO_CHART_URL, symbol);

        let response = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .with_context(|| format!("Failed to fetch quote for {}", symbol))?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Yahoo Finance API returned error for {}: {}",
                symbol,
                response.status()
            );
        }

        let data: ChartResponse = response
            .json()
            .await
            .with_context(|| format!("Failed to parse response for {}", symbol))?;

        // Check for API errors
        if let Some(error) = data.chart.error {
            anyhow::bail!("Yahoo Finance error for {}: {}", symbol, error.description);
        }

        let result = data
            .chart
            .result
            .and_then(|r| r.into_iter().next())
            .ok_or_else(|| anyhow::anyhow!("No data returned for {}", symbol))?;

        Ok(result.into_quote())
    }

    /// Fetch a single quote.
    /// For when you only need to be disappointed by one stock at a time.
    #[allow(dead_code)] // Reserved for future regret-checking functionality
    pub async fn get_quote(&self, symbol: &str) -> Result<Quote> {
        self.fetch_single_quote(symbol).await
    }
}

impl Default for YahooFinanceClient {
    fn default() -> Self {
        Self::new(10).expect("Failed to create default client")
    }
}

// Yahoo Finance v8 Chart API response structures

#[derive(Debug, Deserialize)]
struct ChartResponse {
    chart: ChartData,
}

#[derive(Debug, Deserialize)]
struct ChartData {
    result: Option<Vec<ChartResult>>,
    error: Option<ChartError>,
}

#[derive(Debug, Deserialize)]
struct ChartError {
    #[allow(dead_code)]
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChartResult {
    meta: ChartMeta,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChartMeta {
    symbol: String,
    #[serde(default)]
    short_name: Option<String>,
    #[serde(default)]
    long_name: Option<String>,
    #[serde(default)]
    regular_market_price: Option<f64>,
    #[serde(default)]
    chart_previous_close: Option<f64>,
    #[serde(default)]
    previous_close: Option<f64>,
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
    currency: Option<String>,
    #[serde(default)]
    exchange_name: Option<String>,
    #[serde(default)]
    instrument_type: Option<String>,
    #[serde(default)]
    regular_market_time: Option<i64>,
}

impl ChartResult {
    fn into_quote(self) -> Quote {
        let meta = self.meta;
        let prev_close = meta
            .chart_previous_close
            .or(meta.previous_close)
            .unwrap_or(0.0);
        let price = meta.regular_market_price.unwrap_or(0.0);
        let change = price - prev_close;
        let change_percent = if prev_close > 0.0 {
            (change / prev_close) * 100.0
        } else {
            0.0
        };

        Quote {
            symbol: meta.symbol,
            name: meta
                .short_name
                .or(meta.long_name)
                .unwrap_or_else(|| "Unknown".to_string()),
            price,
            change,
            change_percent,
            previous_close: prev_close,
            open: 0.0, // Not available in chart API meta
            day_high: meta.regular_market_day_high.unwrap_or(0.0),
            day_low: meta.regular_market_day_low.unwrap_or(0.0),
            year_high: meta.fifty_two_week_high.unwrap_or(0.0),
            year_low: meta.fifty_two_week_low.unwrap_or(0.0),
            volume: meta.regular_market_volume.unwrap_or(0),
            avg_volume: 0,    // Not available in chart API meta
            market_cap: None, // Not available in chart API meta
            currency: meta.currency.unwrap_or_else(|| "USD".to_string()),
            exchange: meta.exchange_name.unwrap_or_default(),
            quote_type: parse_quote_type(meta.instrument_type.as_deref()),
            market_state: MarketState::Closed, // Would need separate call to determine
            timestamp: meta
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
