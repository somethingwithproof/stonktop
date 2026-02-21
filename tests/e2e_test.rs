//! End-to-end tests for the stonktop pipeline.
//!
//! These tests exercise the full stack from App construction through API
//! parsing using wiremock to stand in for Yahoo Finance. No real network
//! calls except where #[ignore] is present.

use clap::Parser;
use std::io::Write;
use std::time::Duration;
use stonktop::app::App;
use stonktop::cli::Args;
use stonktop::config::{Config, HoldingConfig};
use stonktop::models::{Quote, QuoteType, SortDirection, SortOrder};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// --- fixtures ---

const AAPL_FIXTURE: &str = include_str!("fixtures/aapl_success.json");
const BTC_FIXTURE: &str = include_str!("fixtures/btc_usd_success.json");
const ERR_FIXTURE: &str = include_str!("fixtures/error_not_found.json");

// --- helpers ---

fn base_args(symbols: &[&str]) -> Args {
    let mut argv = vec!["stonktop", "-b", "-n", "1"];
    if !symbols.is_empty() {
        argv.push("-s");
        argv.push(symbols[0]);
        // clap comma-splits, but for multiple distinct symbols use repeated -s;
        // easier to just join and let clap value_delimiter handle it
    }
    // Build the full symbol string for the -s flag (clap splits on comma)
    let joined;
    if symbols.len() > 1 {
        joined = symbols.join(",");
        // rebuild without the placeholder
        let mut a = vec!["stonktop", "-b", "-n", "1", "-s"];
        a.push(&joined);
        return Args::parse_from(a);
    }
    Args::parse_from(argv)
}

fn args_with_timeout(symbols: &[&str], timeout: u64) -> Args {
    let joined = symbols.join(",");
    Args::parse_from([
        "stonktop",
        "-b",
        "-n",
        "1",
        "-s",
        &joined,
        "--timeout",
        &timeout.to_string(),
    ])
}

fn stonktop_bin() -> std::process::Command {
    std::process::Command::new(env!("CARGO_BIN_EXE_stonktop"))
}

// --- pipeline tests ---

#[tokio::test]
async fn test_single_symbol_pipeline() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(AAPL_FIXTURE))
        .mount(&server)
        .await;

    let args = base_args(&["AAPL"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    assert_eq!(app.quotes.len(), 1, "expected exactly one quote");
    let q = &app.quotes[0];
    assert_eq!(q.symbol, "AAPL");
    assert!(
        (q.price - 195.89).abs() < 0.01,
        "price mismatch: {}",
        q.price
    );
    assert!(
        (q.previous_close - 192.53).abs() < 0.01,
        "prev_close mismatch: {}",
        q.previous_close
    );
    assert_eq!(q.quote_type, QuoteType::Equity);
    assert_eq!(app.error, None);
}

#[tokio::test]
async fn test_multi_symbol_pipeline() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(AAPL_FIXTURE))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/BTC-USD"))
        .respond_with(ResponseTemplate::new(200).set_body_string(BTC_FIXTURE))
        .mount(&server)
        .await;

    let args = base_args(&["AAPL", "BTC-USD"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    assert_eq!(app.quotes.len(), 2, "expected two quotes");

    let mut symbols: Vec<&str> = app.quotes.iter().map(|q| q.symbol.as_str()).collect();
    symbols.sort_unstable();
    assert_eq!(symbols, ["AAPL", "BTC-USD"]);

    let btc = app.quotes.iter().find(|q| q.symbol == "BTC-USD").unwrap();
    assert!((btc.price - 43250.75).abs() < 0.01);
    assert_eq!(btc.quote_type, QuoteType::Cryptocurrency);
}

#[tokio::test]
async fn test_crypto_expansion_e2e() {
    // Pass "BTC"; App::build expands it to "BTC-USD" before issuing the request.
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/BTC-USD"))
        .respond_with(ResponseTemplate::new(200).set_body_string(BTC_FIXTURE))
        .mount(&server)
        .await;

    let args = base_args(&["BTC"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    // Symbol should be expanded in app.symbols before any request
    assert_eq!(app.symbols, vec!["BTC-USD"]);

    app.refresh().await.unwrap();

    assert_eq!(app.quotes.len(), 1);
    assert_eq!(app.quotes[0].symbol, "BTC-USD");
}

#[tokio::test]
async fn test_portfolio_calculations() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(AAPL_FIXTURE))
        .mount(&server)
        .await;

    let config = Config {
        holdings: vec![HoldingConfig {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            cost_basis: 150.0,
        }],
        ..Config::default()
    };

    let args = base_args(&["AAPL"]);
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    // price = 195.89, qty = 10 → value = 1958.90
    let value = app.total_portfolio_value();
    assert!(
        (value - 1958.90).abs() < 0.10,
        "portfolio value mismatch: {}",
        value
    );

    // pnl = value - cost = 1958.90 - 1500.00 = 458.90
    let pnl = app.total_portfolio_pnl();
    assert!(pnl > 0.0, "expected positive pnl, got {}", pnl);
    assert!((pnl - 458.90).abs() < 0.10, "pnl mismatch: {}", pnl);

    // today change = qty * change; change = 195.89 - 192.53 = 3.36 → 33.60
    let today = app.today_portfolio_change();
    assert!(
        (today - 33.60).abs() < 0.10,
        "today_change mismatch: {}",
        today
    );
}

#[tokio::test]
async fn test_batch_output_via_binary() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(AAPL_FIXTURE))
        .mount(&server)
        .await;

    let output = stonktop_bin()
        .args(["-s", "AAPL", "-b", "-n", "1"])
        .env("STONKTOP_API_BASE_URL", server.uri())
        .output()
        .expect("failed to run stonktop binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "binary exited non-zero; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("AAPL"), "stdout missing AAPL:\n{}", stdout);
}

// --- error handling tests ---

#[tokio::test]
async fn test_api_404() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/INVALID"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let args = base_args(&["INVALID"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    // get_quotes silently skips failures; no panic, no quote
    assert_eq!(app.quotes.len(), 0, "expected no quotes on 404");
}

#[tokio::test]
async fn test_api_timeout() {
    let server = MockServer::start().await;

    // Delay longer than the 1s client timeout
    Mock::given(method("GET"))
        .and(path("/SLOW"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(AAPL_FIXTURE)
                .set_delay(Duration::from_secs(30)),
        )
        .mount(&server)
        .await;

    let args = args_with_timeout(&["SLOW"], 1);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    // refresh() absorbs the per-symbol error; app.error is set for total failure
    // but individual timeout is swallowed by the filter_map in get_quotes.
    // Either way: no panic, no quote.
    app.refresh().await.unwrap();
    assert_eq!(app.quotes.len(), 0, "expected no quotes on timeout");
}

#[tokio::test]
async fn test_malformed_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string("this is not json }{"))
        .mount(&server)
        .await;

    let args = base_args(&["AAPL"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    assert_eq!(app.quotes.len(), 0, "expected no quotes on malformed JSON");
}

#[tokio::test]
async fn test_yahoo_error_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/DELISTED"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ERR_FIXTURE))
        .mount(&server)
        .await;

    let args = base_args(&["DELISTED"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    // API-level error in the JSON body; get_quotes drops it
    assert_eq!(app.quotes.len(), 0, "expected no quotes for error response");
}

#[tokio::test]
async fn test_partial_failure() {
    // AAPL succeeds; INVALID returns 404. AAPL quote must survive.
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(AAPL_FIXTURE))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/INVALID"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let args = base_args(&["AAPL", "INVALID"]);
    let config = Config::default();
    let mut app = App::with_base_url(&args, &config, server.uri()).unwrap();

    app.refresh().await.unwrap();

    assert_eq!(app.quotes.len(), 1, "expected only the successful quote");
    assert_eq!(app.quotes[0].symbol, "AAPL");
}

// --- data processing tests ---

#[test]
fn test_sort_by_price() {
    use chrono::Utc;
    use stonktop::models::{MarketState, QuoteType};

    let args = Args::parse_from(["stonktop", "-s", "AAPL", "-b", "-n", "1", "-o", "price"]);
    let config = Config::default();
    // with_base_url with a dummy URL; no network call happens
    let mut app = App::with_base_url(&args, &config, "http://127.0.0.1:1".to_string()).unwrap();

    // Push quotes in non-sorted order
    let make_quote = |sym: &str, price: f64| Quote {
        symbol: sym.to_string(),
        name: sym.to_string(),
        price,
        change: 0.0,
        change_percent: 0.0,
        previous_close: price,
        open: price,
        day_high: price,
        day_low: price,
        year_high: price,
        year_low: price,
        volume: 0,
        avg_volume: 0,
        market_cap: None,
        currency: "USD".to_string(),
        exchange: String::new(),
        quote_type: QuoteType::Equity,
        market_state: MarketState::Closed,
        timestamp: Utc::now(),
    };

    app.quotes.push(make_quote("MID", 150.0));
    app.quotes.push(make_quote("LOW", 50.0));
    app.quotes.push(make_quote("HIGH", 500.0));

    // Default sort_direction for non-reverse args is Descending
    app.sort_direction = SortDirection::Descending;
    app.sort_order = SortOrder::Price;
    app.sort_quotes();

    let prices: Vec<f64> = app.quotes.iter().map(|q| q.price).collect();
    assert_eq!(prices, vec![500.0, 150.0, 50.0], "prices not descending");

    app.sort_direction = SortDirection::Ascending;
    app.sort_quotes();

    let prices: Vec<f64> = app.quotes.iter().map(|q| q.price).collect();
    assert_eq!(prices, vec![50.0, 150.0, 500.0], "prices not ascending");
}

#[test]
fn test_config_loading_pipeline() {
    use tempfile::NamedTempFile;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(
        tmp,
        r#"
[watchlist]
symbols = ["AAPL"]

[[holdings]]
symbol = "AAPL"
quantity = 5.0
cost_basis = 100.0
"#
    )
    .unwrap();

    let config = Config::load(&tmp.path().to_path_buf()).unwrap();

    assert_eq!(config.watchlist.symbols, vec!["AAPL"]);
    assert_eq!(config.holdings.len(), 1);
    assert!((config.holdings[0].quantity - 5.0).abs() < f64::EPSILON);
    assert!((config.holdings[0].cost_basis - 100.0).abs() < f64::EPSILON);

    // Ensure App can be built from this config without network calls
    let args = Args::parse_from(["stonktop", "-b", "-n", "1"]);
    let app = App::with_base_url(&args, &config, "http://127.0.0.1:1".to_string()).unwrap();

    assert_eq!(app.symbols, vec!["AAPL"]);
    assert!(app.holdings.contains_key("AAPL"));
}

// --- real network smoke test ---

/// Fetch a real AAPL quote from Yahoo Finance.
///
/// Skipped in CI; run with: cargo test -- --ignored
#[tokio::test]
#[ignore]
async fn test_real_yahoo_api() {
    let args = Args::parse_from(["stonktop", "-s", "AAPL", "-b", "-n", "1"]);
    let config = Config::default();
    let mut app = App::new(&args, &config).unwrap();

    app.refresh().await.unwrap();

    assert!(!app.quotes.is_empty(), "expected at least one quote");
    let q = &app.quotes[0];
    assert_eq!(q.symbol, "AAPL");
    assert!(q.price > 0.0, "price must be positive, got {}", q.price);
}
