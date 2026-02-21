//! Application state and logic.
//!
//! Where we keep track of your hopes, dreams, and unrealized losses.

use crate::api::{expand_symbol, YahooFinanceClient};
use crate::cli::Args;
use crate::config::Config;
use crate::models::{Holding, Quote, SortDirection, SortOrder};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Input mode for interactive commands.
/// Normal is for watching numbers move. AddSymbol is for adding more numbers to watch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    AddSymbol,
}

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
    pub verbose: bool,
    /// Color mode preference
    pub color_mode: crate::cli::ColorMode,
    /// Current input mode
    pub input_mode: InputMode,
    /// Input buffer for text entry
    pub input_buffer: String,
    /// Show detail popup for selected quote
    pub show_detail: bool,
}

impl App {
    /// Create a new application from CLI args and config.
    pub fn new(args: &Args, config: &Config) -> Result<Self> {
        let client = YahooFinanceClient::new(args.timeout)?;
        Self::build(args, config, client)
    }

    /// Create a new application with a custom API base URL (for testing).
    #[allow(dead_code)] // Used by e2e and unit tests via lib crate
    pub fn with_base_url(args: &Args, config: &Config, base_url: String) -> Result<Self> {
        let client = YahooFinanceClient::with_base_url(args.timeout, base_url)?;
        Self::build(args, config, client)
    }

    fn build(args: &Args, config: &Config, client: YahooFinanceClient) -> Result<Self> {
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
            color_mode: crate::cli::ColorMode::default(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            show_detail: false,
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
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            }
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
        self.scroll_offset = 0;
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

    /// Toggle detail view for the selected quote.
    pub fn toggle_detail(&mut self) {
        if !self.secure_mode {
            self.show_detail = !self.show_detail;
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
    pub fn add_symbol(&mut self, symbol: &str) {
        let expanded = expand_symbol(symbol);
        if !self.symbols.contains(&expanded) {
            self.symbols.push(expanded);
        }
    }

    /// Remove a symbol from watch.
    /// Denial is the first stage of grief. Removing it from your watchlist is the second.
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

    /// Handle keyboard input.
    pub fn handle_key_event(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Close detail view on any key
        if self.show_detail {
            self.show_detail = false;
            return;
        }

        // Handle symbol input mode
        if self.input_mode == InputMode::AddSymbol {
            match code {
                KeyCode::Enter => {
                    if !self.input_buffer.is_empty() {
                        let symbol = self.input_buffer.drain(..).collect::<String>();
                        self.add_symbol(&symbol.to_uppercase());
                        self.last_refresh = None; // trigger refresh
                    }
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    self.input_buffer.clear();
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                _ => {}
            }
            return;
        }

        // Close help overlay on any key
        if self.show_help {
            self.show_help = false;
            return;
        }

        // Clear error on any key
        if self.error.is_some() {
            self.error = None;
            return;
        }

        match code {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => self.quit(),

            // Navigation
            KeyCode::Up | KeyCode::Char('k') => self.select_up(),
            KeyCode::Down | KeyCode::Char('j') => self.select_down(),
            KeyCode::Home | KeyCode::Char('g') => self.select_top(),
            KeyCode::End | KeyCode::Char('G') => self.select_bottom(),
            KeyCode::PageUp => {
                for _ in 0..10 {
                    self.select_up();
                }
            }
            KeyCode::PageDown => {
                for _ in 0..10 {
                    self.select_down();
                }
            }

            // Sorting
            KeyCode::Char('s') => self.next_sort_order(),
            KeyCode::Char('r') => self.toggle_sort_direction(),
            KeyCode::Char('1') => self.set_sort_order(SortOrder::Symbol),
            KeyCode::Char('2') => self.set_sort_order(SortOrder::Name),
            KeyCode::Char('3') => self.set_sort_order(SortOrder::Price),
            KeyCode::Char('4') => self.set_sort_order(SortOrder::Change),
            KeyCode::Char('5') => self.set_sort_order(SortOrder::ChangePercent),
            KeyCode::Char('6') => self.set_sort_order(SortOrder::Volume),
            KeyCode::Char('7') => self.set_sort_order(SortOrder::MarketCap),

            // Display toggles
            KeyCode::Char('H') => self.toggle_holdings(),
            KeyCode::Char('f') => self.toggle_fundamentals(),
            KeyCode::Char('h') | KeyCode::Char('?') => self.toggle_help(),

            // Symbol management
            KeyCode::Char('a') => {
                self.input_mode = InputMode::AddSymbol;
                self.input_buffer.clear();
            }
            KeyCode::Char('d') => {
                if let Some(quote) = self.selected_quote() {
                    let symbol = quote.symbol.clone();
                    self.remove_symbol(&symbol);
                }
            }

            // Detail view
            KeyCode::Enter => self.toggle_detail(),

            // Refresh
            KeyCode::Char(' ') | KeyCode::Char('R') => {
                self.last_refresh = None; // Force refresh on next tick
            }

            // Groups
            KeyCode::Tab => {
                if !self.groups.is_empty() {
                    self.active_group = (self.active_group + 1) % self.groups.len();
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Args;
    use crate::config::Config;
    use crate::models::{Quote, SortOrder};
    use clap::Parser;
    use crossterm::event::{KeyCode, KeyModifiers};

    fn test_app() -> App {
        let args = Args::parse_from(["stonktop", "-s", "AAPL,GOOGL", "-b", "-n", "1"]);
        let config = Config::default();
        let mut app = App::with_base_url(&args, &config, "http://127.0.0.1:1".to_string()).unwrap();
        // Seed with test quotes so navigation has something to work with
        app.quotes = vec![
            Quote {
                symbol: "AAPL".into(),
                name: "Apple".into(),
                price: 195.0,
                change: 3.0,
                change_percent: 1.5,
                ..Quote::default()
            },
            Quote {
                symbol: "GOOGL".into(),
                name: "Alphabet".into(),
                price: 140.0,
                change: -2.0,
                change_percent: -1.4,
                ..Quote::default()
            },
        ];
        app
    }

    #[test]
    fn test_quit_q() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(!app.running);
    }

    #[test]
    fn test_quit_ctrl_c() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(!app.running);
    }

    #[test]
    fn test_navigation_j_k() {
        let mut app = test_app();
        assert_eq!(app.selected, 0);
        app.handle_key_event(KeyCode::Char('j'), KeyModifiers::NONE);
        assert_eq!(app.selected, 1);
        app.handle_key_event(KeyCode::Char('k'), KeyModifiers::NONE);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigation_g_and_big_g() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('G'), KeyModifiers::NONE);
        assert_eq!(app.selected, 1); // last
        app.handle_key_event(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(app.selected, 0); // first
    }

    #[test]
    fn test_sort_cycle() {
        let mut app = test_app();
        let initial = app.sort_order;
        app.handle_key_event(KeyCode::Char('s'), KeyModifiers::NONE);
        assert_ne!(app.sort_order, initial);
    }

    #[test]
    fn test_sort_reverse() {
        let mut app = test_app();
        let initial = app.sort_direction;
        app.handle_key_event(KeyCode::Char('r'), KeyModifiers::NONE);
        assert_ne!(app.sort_direction, initial);
    }

    #[test]
    fn test_sort_number_keys() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('3'), KeyModifiers::NONE);
        assert_eq!(app.sort_order, SortOrder::Price);
        app.handle_key_event(KeyCode::Char('1'), KeyModifiers::NONE);
        assert_eq!(app.sort_order, SortOrder::Symbol);
    }

    #[test]
    fn test_toggle_holdings() {
        let mut app = test_app();
        assert!(!app.show_holdings);
        app.handle_key_event(KeyCode::Char('H'), KeyModifiers::NONE);
        assert!(app.show_holdings);
    }

    #[test]
    fn test_toggle_help() {
        let mut app = test_app();
        assert!(!app.show_help);
        app.handle_key_event(KeyCode::Char('h'), KeyModifiers::NONE);
        assert!(app.show_help);
    }

    #[test]
    fn test_help_dismissal() {
        let mut app = test_app();
        app.show_help = true;
        app.handle_key_event(KeyCode::Char('x'), KeyModifiers::NONE); // any key
        assert!(!app.show_help);
    }

    #[test]
    fn test_error_dismissal() {
        let mut app = test_app();
        app.error = Some("test error".to_string());
        app.handle_key_event(KeyCode::Char('x'), KeyModifiers::NONE);
        assert!(app.error.is_none());
    }

    #[test]
    fn test_detail_toggle() {
        let mut app = test_app();
        assert!(!app.show_detail);
        app.handle_key_event(KeyCode::Enter, KeyModifiers::NONE);
        assert!(app.show_detail);
        // Any key dismisses
        app.handle_key_event(KeyCode::Char('x'), KeyModifiers::NONE);
        assert!(!app.show_detail);
    }

    #[test]
    fn test_add_symbol_mode() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(app.input_mode, InputMode::AddSymbol);

        // Type "MSFT"
        app.handle_key_event(KeyCode::Char('m'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('s'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('f'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('t'), KeyModifiers::NONE);
        assert_eq!(app.input_buffer, "msft");

        // Enter confirms
        app.handle_key_event(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.symbols.contains(&"MSFT".to_string()));
    }

    #[test]
    fn test_add_symbol_cancel() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('x'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.input_buffer.is_empty());
    }

    #[test]
    fn test_remove_symbol() {
        let mut app = test_app();
        assert_eq!(app.selected, 0);
        let initial_count = app.quotes.len();
        app.handle_key_event(KeyCode::Char('d'), KeyModifiers::NONE);
        assert_eq!(app.quotes.len(), initial_count - 1);
    }

    // --- Navigation edge cases ---

    #[test]
    fn test_select_up_at_zero() {
        let mut app = test_app();
        app.selected = 0;
        app.select_up();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_select_down_at_end() {
        let mut app = test_app();
        app.selected = app.quotes.len() - 1;
        app.select_down();
        assert_eq!(app.selected, app.quotes.len() - 1);
    }

    #[test]
    fn test_select_top_bottom_empty() {
        let mut app = test_app();
        app.quotes.clear();
        app.select_top();
        assert_eq!(app.selected, 0);
        app.select_bottom();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_scroll_offset_updates() {
        let mut app = test_app();
        app.scroll_offset = 1;
        app.selected = 1;
        app.select_up(); // selected=0, should update scroll_offset
        assert_eq!(app.scroll_offset, 0);
    }

    // --- Portfolio math edge cases ---

    #[test]
    fn test_portfolio_zero_holdings() {
        let mut app = test_app();
        app.holdings.clear();
        assert_eq!(app.total_portfolio_value(), 0.0);
        assert_eq!(app.total_portfolio_cost(), 0.0);
        assert_eq!(app.total_portfolio_pnl(), 0.0);
        assert_eq!(app.today_portfolio_change(), 0.0);
    }
}
