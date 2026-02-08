//! Application state and logic.
//!
//! Where we keep track of your hopes, dreams, and unrealized losses.

use crate::api::{expand_symbol, YahooFinanceClient};
use crate::cli::Args;
use crate::config::Config;
use crate::models::{Holding, Quote, SortDirection, SortOrder};
use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Application state.
/// Think of it as your financial life, but with better error handling.
pub struct App {
    /// Current quotes
    pub quotes: Vec<Quote>,
    /// Holdings/portfolio
    pub holdings: HashMap<String, Holding>,
    /// Symbols being watched
    pub symbols: Vec<String>,
    /// API client
    client: YahooFinanceClient,
    /// Last refresh time
    pub last_refresh: Option<Instant>,
    /// Refresh interval
    pub refresh_interval: Duration,
    /// Current sort order
    pub sort_order: SortOrder,
    /// Sort direction
    pub sort_direction: SortDirection,
    /// Current iteration count
    pub iteration: u64,
    /// Maximum iterations (0 = infinite)
    pub max_iterations: u64,
    /// Is the app running
    pub running: bool,
    /// Error message to display
    pub error: Option<String>,
    /// Selected row index
    pub selected: usize,
    /// Scroll offset for when you have more regrets than fit on screen
    #[allow(dead_code)] // Future scrolling feature - coming soon to a terminal near you
    pub scroll_offset: usize,
    /// Show help overlay
    pub show_help: bool,
    /// Show holdings view
    pub show_holdings: bool,
    /// Show fundamentals
    pub show_fundamentals: bool,
    /// Batch mode (non-interactive)
    pub batch_mode: bool,
    /// Secure mode (no interactive commands)
    pub secure_mode: bool,
    /// Active group index
    pub active_group: usize,
    /// Group names
    pub groups: Vec<String>,
    /// Verbose mode - for when you want MORE numbers to stress about
    #[allow(dead_code)] // TODO: Add more verbosity, because anxiety needs details
    pub verbose: bool,
}

impl App {
    /// Create a new application from CLI args and config.
    pub fn new(args: &Args, config: &Config) -> Result<Self> {
        // Merge symbols from args and config
        let mut symbols: Vec<String> = args.symbols.clone().unwrap_or_else(|| config.all_symbols());

        // Expand symbol shortcuts
        symbols = symbols.into_iter().map(|s| expand_symbol(&s)).collect();

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        symbols.retain(|s| seen.insert(s.clone()));

        // Build holdings map
        let holdings: HashMap<String, Holding> = config
            .get_holdings()
            .into_iter()
            .map(|h| (expand_symbol(&h.symbol), h))
            .collect();

        // Get groups
        let groups: Vec<String> = config.groups.keys().cloned().collect();

        let client = YahooFinanceClient::new(args.timeout)?;

        // Enforce minimum refresh interval of 1.0 second
        let delay = if args.delay < 1.0 { 1.0 } else { args.delay };

        Ok(Self {
            quotes: Vec::new(),
            holdings,
            symbols,
            client,
            last_refresh: None,
            refresh_interval: Duration::from_secs_f64(delay),
            sort_order: args.sort.into(),
            sort_direction: if args.reverse {
                SortDirection::Ascending
            } else {
                SortDirection::Descending
            },
            iteration: 0,
            max_iterations: args.iterations,
            running: true,
            error: None,
            selected: 0,
            scroll_offset: 0,
            show_help: false,
            show_holdings: args.holdings || config.display.show_holdings,
            show_fundamentals: config.display.show_fundamentals,
            batch_mode: args.batch,
            secure_mode: args.secure,
            active_group: 0,
            groups,
            verbose: args.verbose,
        })
    }

    /// Check if refresh is needed.
    pub fn needs_refresh(&self) -> bool {
        match self.last_refresh {
            None => true,
            Some(last) => last.elapsed() >= self.refresh_interval,
        }
    }

    /// Refresh quotes from API.
    pub async fn refresh(&mut self) -> Result<()> {
        if self.symbols.is_empty() {
            return Ok(());
        }

        match self.client.get_quotes(&self.symbols).await {
            Ok(quotes) => {
                self.quotes = quotes;
                self.sort_quotes();
                self.last_refresh = Some(Instant::now());
                self.iteration += 1;
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("API Error: {}", e));
            }
        }

        Ok(())
    }

    /// Sort quotes according to current sort settings.
    pub fn sort_quotes(&mut self) {
        let direction = self.sort_direction;

        self.quotes.sort_by(|a, b| {
            let cmp = match self.sort_order {
                SortOrder::Symbol => a.symbol.cmp(&b.symbol),
                SortOrder::Name => a.name.cmp(&b.name),
                SortOrder::Price => a
                    .price
                    .partial_cmp(&b.price)
                    .unwrap_or(std::cmp::Ordering::Equal),
                SortOrder::Change => a
                    .change
                    .partial_cmp(&b.change)
                    .unwrap_or(std::cmp::Ordering::Equal),
                SortOrder::ChangePercent => a
                    .change_percent
                    .partial_cmp(&b.change_percent)
                    .unwrap_or(std::cmp::Ordering::Equal),
                SortOrder::Volume => a.volume.cmp(&b.volume),
                SortOrder::MarketCap => a.market_cap.cmp(&b.market_cap),
            };

            match direction {
                SortDirection::Ascending => cmp,
                SortDirection::Descending => cmp.reverse(),
            }
        });
    }

    /// Toggle sort direction.
    pub fn toggle_sort_direction(&mut self) {
        self.sort_direction = self.sort_direction.toggle();
        self.sort_quotes();
    }

    /// Cycle to next sort order.
    pub fn next_sort_order(&mut self) {
        self.sort_order = self.sort_order.next();
        self.sort_quotes();
    }

    /// Set specific sort order.
    pub fn set_sort_order(&mut self, order: SortOrder) {
        if self.sort_order == order {
            self.toggle_sort_direction();
        } else {
            self.sort_order = order;
            self.sort_direction = SortDirection::Descending;
        }
        self.sort_quotes();
    }

    /// Move selection up.
    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down.
    pub fn select_down(&mut self) {
        if self.selected < self.quotes.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Move selection to top.
    pub fn select_top(&mut self) {
        self.selected = 0;
    }

    /// Move selection to bottom.
    pub fn select_bottom(&mut self) {
        self.selected = self.quotes.len().saturating_sub(1);
    }

    /// Toggle help display.
    pub fn toggle_help(&mut self) {
        if !self.secure_mode {
            self.show_help = !self.show_help;
        }
    }

    /// Toggle holdings view.
    pub fn toggle_holdings(&mut self) {
        if !self.secure_mode {
            self.show_holdings = !self.show_holdings;
        }
    }

    /// Toggle fundamentals display.
    pub fn toggle_fundamentals(&mut self) {
        if !self.secure_mode {
            self.show_fundamentals = !self.show_fundamentals;
        }
    }

    /// Quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Check if max iterations reached.
    pub fn should_quit(&self) -> bool {
        !self.running || (self.max_iterations > 0 && self.iteration >= self.max_iterations)
    }

    /// Get total portfolio value.
    pub fn total_portfolio_value(&self) -> f64 {
        self.quotes
            .iter()
            .filter_map(|q| {
                self.holdings
                    .get(&q.symbol)
                    .map(|h| h.current_value(q.price))
            })
            .sum()
    }

    /// Get total portfolio cost.
    pub fn total_portfolio_cost(&self) -> f64 {
        self.holdings.values().map(|h| h.total_cost()).sum()
    }

    /// Get total portfolio profit/loss.
    pub fn total_portfolio_pnl(&self) -> f64 {
        self.total_portfolio_value() - self.total_portfolio_cost()
    }

    /// Get today's portfolio change.
    pub fn today_portfolio_change(&self) -> f64 {
        self.quotes
            .iter()
            .filter_map(|q| self.holdings.get(&q.symbol).map(|h| h.quantity * q.change))
            .sum()
    }

    /// Add a symbol to watch.
    /// For when FOMO hits and you need to track one more meme stock.
    #[allow(dead_code)] // Interactive symbol adding - coming in v2.0 (probably)
    pub fn add_symbol(&mut self, symbol: &str) {
        let expanded = expand_symbol(symbol);
        if !self.symbols.contains(&expanded) {
            self.symbols.push(expanded);
        }
    }

    /// Remove a symbol from watch.
    /// Denial is the first stage of grief. Removing it from your watchlist is the second.
    #[allow(dead_code)] // Interactive symbol removal - for those who can't handle the truth
    pub fn remove_symbol(&mut self, symbol: &str) {
        let expanded = expand_symbol(symbol);
        self.symbols.retain(|s| s != &expanded);
        self.quotes.retain(|q| q.symbol != expanded);
        if self.selected >= self.quotes.len() {
            self.selected = self.quotes.len().saturating_sub(1);
        }
    }

    /// Get the currently selected quote.
    /// Returns the quote you're currently staring at in disbelief.
    #[allow(dead_code)] // Used by future detail view feature
    pub fn selected_quote(&self) -> Option<&Quote> {
        self.quotes.get(self.selected)
    }

    /// Get time since last refresh as human readable string.
    pub fn time_since_refresh(&self) -> String {
        match self.last_refresh {
            Some(t) => {
                let elapsed = t.elapsed().as_secs();
                if elapsed < 60 {
                    format!("{}s ago", elapsed)
                } else {
                    format!("{}m ago", elapsed / 60)
                }
            }
            None => "never".to_string(),
        }
    }
}
