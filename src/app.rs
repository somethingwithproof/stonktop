//! Application state and logic.
//!
//! Where we keep track of your hopes, dreams, and unrealized losses.

use crate::api::{expand_symbol_with, YahooFinanceClient};
use crate::cli::Args;
use crate::config::Config;
use crate::models::{Alert, Holding, Quote, QuoteProvider, SortDirection, SortOrder};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Input mode for interactive commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    AddSymbol,
    Search,
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
    /// API client (port — any QuoteProvider impl)
    client: Box<dyn QuoteProvider>,
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
    /// Group names ("All" is prepended automatically)
    pub groups: Vec<String>,
    /// Group symbol lists (index matches groups; index 0 = all symbols)
    pub group_symbols: Vec<Vec<String>>,
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
    /// Color configuration from config file
    pub color_config: crate::config::ColorConfig,
    /// Search/filter string
    pub search_filter: String,
    /// Price alerts
    pub alerts: Vec<Alert>,
    /// Triggered alerts (symbol, message)
    pub triggered_alerts: Vec<(String, String)>,
    /// Config file path for persistent saves
    pub config_path: Option<std::path::PathBuf>,
    /// Configurable crypto shortcuts from config
    pub crypto_shortcuts: HashMap<String, String>,
}

impl App {
    /// Create a new application from CLI args and config.
    pub fn new(args: &Args, config: &Config) -> Result<Self> {
        let client = YahooFinanceClient::new(args.timeout)?;
        Self::build(args, config, Box::new(client))
    }

    /// Create a new application with a custom API base URL (for testing).
    #[allow(dead_code)] // Used by e2e and unit tests via lib crate
    pub fn with_base_url(args: &Args, config: &Config, base_url: String) -> Result<Self> {
        let client = YahooFinanceClient::with_base_url(args.timeout, base_url)?;
        Self::build(args, config, Box::new(client))
    }

    /// Create a new application with a custom QuoteProvider (for testing/plugins).
    #[allow(dead_code)]
    pub fn with_provider(args: &Args, config: &Config, client: Box<dyn QuoteProvider>) -> Result<Self> {
        Self::build(args, config, client)
    }

    fn build(args: &Args, config: &Config, client: Box<dyn QuoteProvider>) -> Result<Self> {
        let custom_shortcuts = config.shortcuts.clone();

        // Merge symbols from args and config
        let mut symbols: Vec<String> = args.symbols.clone().unwrap_or_else(|| config.all_symbols());

        // Expand symbol shortcuts (custom first, then built-in)
        symbols = symbols
            .into_iter()
            .map(|s| expand_symbol_with(&s, &custom_shortcuts))
            .collect();

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        symbols.retain(|s| seen.insert(s.clone()));

        // Build holdings map
        let holdings: HashMap<String, Holding> = config
            .get_holdings()
            .into_iter()
            .map(|h| (expand_symbol_with(&h.symbol, &custom_shortcuts), h))
            .collect();

        // Build groups: first entry is "All", then each config group
        let mut groups = vec!["All".to_string()];
        let mut group_symbols: Vec<Vec<String>> = vec![symbols.clone()];
        for (name, syms) in &config.groups {
            groups.push(name.clone());
            group_symbols.push(
                syms.iter()
                    .map(|s| expand_symbol_with(s, &custom_shortcuts))
                    .collect(),
            );
        }

        // Build alerts
        let alerts: Vec<Alert> = config
            .alerts
            .iter()
            .map(|a| Alert {
                symbol: expand_symbol_with(&a.symbol, &custom_shortcuts),
                above: a.above,
                below: a.below,
            })
            .collect();
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
            group_symbols,
            verbose: args.verbose,
            color_mode: args.color,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            show_detail: false,
            color_config: config.colors.clone(),
            search_filter: String::new(),
            alerts,
            triggered_alerts: Vec::new(),
            config_path: Config::default_config_path(),
            crypto_shortcuts: custom_shortcuts,
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
                self.check_alerts();
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
        let max = self.visible_quotes().len().saturating_sub(1);
        if self.selected < max {
            self.selected += 1;
            // Keep the selected row visible (assume ~20 visible rows)
            let visible_rows = 20usize;
            if self.selected >= self.scroll_offset + visible_rows {
                self.scroll_offset = self.selected - visible_rows + 1;
            }
        }
    }

    /// Move selection to top.
    pub fn select_top(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Move selection to bottom.
    pub fn select_bottom(&mut self) {
        self.selected = self.visible_quotes().len().saturating_sub(1);
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
    pub fn add_symbol(&mut self, symbol: &str) {
        let expanded = expand_symbol_with(symbol, &self.crypto_shortcuts);
        if !self.symbols.contains(&expanded) {
            self.symbols.push(expanded);
        }
    }

    /// Remove a symbol from watch.
    pub fn remove_symbol(&mut self, symbol: &str) {
        let expanded = expand_symbol_with(symbol, &self.crypto_shortcuts);
        self.symbols.retain(|s| s != &expanded);
        self.quotes.retain(|q| q.symbol != expanded);
        if self.selected >= self.quotes.len() {
            self.selected = self.quotes.len().saturating_sub(1);
        }
    }

    /// Get quotes filtered by the active group and search filter.
    pub fn visible_quotes(&self) -> Vec<&Quote> {
        let base: Vec<&Quote> = if self.active_group == 0 || self.active_group >= self.group_symbols.len() {
            self.quotes.iter().collect()
        } else {
            let group_syms = &self.group_symbols[self.active_group];
            self.quotes
                .iter()
                .filter(|q| group_syms.contains(&q.symbol))
                .collect()
        };

        if self.search_filter.is_empty() {
            base
        } else {
            let filter = self.search_filter.to_uppercase();
            base.into_iter()
                .filter(|q| q.symbol.contains(&filter) || q.name.to_uppercase().contains(&filter))
                .collect()
        }
    }

    /// Get the currently selected quote (from visible quotes).
    /// Returns the quote you're currently staring at in disbelief.
    pub fn selected_quote(&self) -> Option<&Quote> {
        let visible = self.visible_quotes();
        visible.get(self.selected).copied()
    }

    /// Get the active group name.
    pub fn active_group_name(&self) -> &str {
        self.groups.get(self.active_group).map_or("All", |s| s.as_str())
    }

    /// Check price alerts and record any that trigger.
    fn check_alerts(&mut self) {
        self.triggered_alerts.clear();
        for alert in &self.alerts {
            if let Some(quote) = self.quotes.iter().find(|q| q.symbol == alert.symbol) {
                if let Some(above) = alert.above {
                    if quote.price >= above {
                        self.triggered_alerts.push((
                            alert.symbol.clone(),
                            format!("{} above ${:.2} (${:.2})", alert.symbol, above, quote.price),
                        ));
                    }
                }
                if let Some(below) = alert.below {
                    if quote.price <= below {
                        self.triggered_alerts.push((
                            alert.symbol.clone(),
                            format!("{} below ${:.2} (${:.2})", alert.symbol, below, quote.price),
                        ));
                    }
                }
            }
        }
    }

    /// Save current watchlist to config file.
    pub fn save_watchlist(&self) -> Result<()> {
        let path = match &self.config_path {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        if !path.exists() {
            return Ok(()); // Don't create config if it doesn't exist
        }
        let content = std::fs::read_to_string(&path)?;
        let mut config: Config = toml::from_str(&content)?;
        config.watchlist.symbols = self.symbols.clone();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(&config)?;
        std::fs::write(&path, serialized)?;
        Ok(())
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

        // Handle search mode
        if self.input_mode == InputMode::Search {
            match code {
                KeyCode::Enter | KeyCode::Esc => {
                    if code == KeyCode::Esc {
                        self.search_filter.clear();
                    }
                    self.input_mode = InputMode::Normal;
                    self.selected = 0;
                    self.scroll_offset = 0;
                }
                KeyCode::Backspace => {
                    self.search_filter.pop();
                    self.selected = 0;
                }
                KeyCode::Char(c) => {
                    self.search_filter.push(c);
                    self.selected = 0;
                }
                _ => {}
            }
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

            // Search/filter
            KeyCode::Char('/') => {
                self.input_mode = InputMode::Search;
                self.search_filter.clear();
            }

            // Detail view
            KeyCode::Enter => self.toggle_detail(),

            // Refresh
            KeyCode::Char(' ') | KeyCode::Char('R') => {
                self.last_refresh = None; // Force refresh on next tick
            }

            // Groups
            KeyCode::Tab => {
                if self.groups.len() > 1 {
                    self.active_group = (self.active_group + 1) % self.groups.len();
                    self.selected = 0;
                    self.scroll_offset = 0;
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

    // --- Search / filter tests ---

    #[test]
    fn test_search_mode_enter() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('/'), KeyModifiers::NONE);
        assert_eq!(app.input_mode, InputMode::Search);
        assert!(app.search_filter.is_empty());
    }

    #[test]
    fn test_search_mode_type_and_filter() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('/'), KeyModifiers::NONE);

        // Type "goo"
        app.handle_key_event(KeyCode::Char('g'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('o'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('o'), KeyModifiers::NONE);

        assert_eq!(app.search_filter, "goo");

        // visible_quotes should filter to GOOGL only (case-insensitive match on name "Alphabet" won't match,
        // but symbol "GOOGL" doesn't match "goo" either... let's check)
        // Actually "GOOGL".contains("GOO") = true (filter is uppercased)
        let visible = app.visible_quotes();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].symbol, "GOOGL");
    }

    #[test]
    fn test_search_mode_backspace() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('/'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('x'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('y'), KeyModifiers::NONE);
        assert_eq!(app.search_filter, "xy");
        app.handle_key_event(KeyCode::Backspace, KeyModifiers::NONE);
        assert_eq!(app.search_filter, "x");
    }

    #[test]
    fn test_search_mode_escape_clears() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('/'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Esc, KeyModifiers::NONE);

        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.search_filter.is_empty(), "Esc should clear search filter");
    }

    #[test]
    fn test_search_mode_enter_keeps_filter() {
        let mut app = test_app();
        app.handle_key_event(KeyCode::Char('/'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
        app.handle_key_event(KeyCode::Enter, KeyModifiers::NONE);

        assert_eq!(app.input_mode, InputMode::Normal);
        assert_eq!(app.search_filter, "a", "Enter should keep the filter active");
    }

    #[test]
    fn test_search_filter_by_name() {
        let mut app = test_app();
        // Filter by name "Alpha" (Alphabet)
        app.search_filter = "alpha".to_string();
        let visible = app.visible_quotes();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].symbol, "GOOGL");
    }

    #[test]
    fn test_search_filter_no_match() {
        let mut app = test_app();
        app.search_filter = "zzzzz".to_string();
        let visible = app.visible_quotes();
        assert!(visible.is_empty());
    }

    // --- Price alerts tests ---

    #[test]
    fn test_check_alerts_above_triggers() {
        let mut app = test_app();
        app.alerts = vec![Alert {
            symbol: "AAPL".to_string(),
            above: Some(190.0), // AAPL price is 195.0
            below: None,
        }];
        app.check_alerts();
        assert_eq!(app.triggered_alerts.len(), 1);
        assert!(app.triggered_alerts[0].1.contains("above"));
    }

    #[test]
    fn test_check_alerts_below_triggers() {
        let mut app = test_app();
        app.alerts = vec![Alert {
            symbol: "GOOGL".to_string(),
            above: None,
            below: Some(150.0), // GOOGL price is 140.0
        }];
        app.check_alerts();
        assert_eq!(app.triggered_alerts.len(), 1);
        assert!(app.triggered_alerts[0].1.contains("below"));
    }

    #[test]
    fn test_check_alerts_no_trigger() {
        let mut app = test_app();
        app.alerts = vec![Alert {
            symbol: "AAPL".to_string(),
            above: Some(300.0), // AAPL is 195, won't trigger
            below: Some(100.0), // AAPL is 195, won't trigger
        }];
        app.check_alerts();
        assert!(app.triggered_alerts.is_empty());
    }

    #[test]
    fn test_check_alerts_both_directions() {
        let mut app = test_app();
        app.alerts = vec![
            Alert {
                symbol: "AAPL".to_string(),
                above: Some(190.0),
                below: None,
            },
            Alert {
                symbol: "GOOGL".to_string(),
                above: None,
                below: Some(150.0),
            },
        ];
        app.check_alerts();
        assert_eq!(app.triggered_alerts.len(), 2);
    }

    #[test]
    fn test_check_alerts_unknown_symbol_ignored() {
        let mut app = test_app();
        app.alerts = vec![Alert {
            symbol: "ZZZZ".to_string(),
            above: Some(1.0),
            below: None,
        }];
        app.check_alerts();
        assert!(app.triggered_alerts.is_empty());
    }

    // --- Group cycling tests ---

    #[test]
    fn test_group_cycling_with_tab() {
        let mut app = test_app();
        app.groups = vec!["All".to_string(), "tech".to_string(), "crypto".to_string()];
        app.group_symbols = vec![
            vec!["AAPL".to_string(), "GOOGL".to_string()],
            vec!["AAPL".to_string()],
            vec!["BTC-USD".to_string()],
        ];
        assert_eq!(app.active_group, 0);
        assert_eq!(app.active_group_name(), "All");

        app.handle_key_event(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.active_group, 1);
        assert_eq!(app.active_group_name(), "tech");

        app.handle_key_event(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.active_group, 2);
        assert_eq!(app.active_group_name(), "crypto");

        // Wraps around
        app.handle_key_event(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.active_group, 0);
        assert_eq!(app.active_group_name(), "All");
    }

    #[test]
    fn test_group_cycling_resets_selection() {
        let mut app = test_app();
        app.groups = vec!["All".to_string(), "tech".to_string()];
        app.group_symbols = vec![
            vec!["AAPL".to_string(), "GOOGL".to_string()],
            vec!["AAPL".to_string()],
        ];
        app.selected = 1;
        app.scroll_offset = 1;

        app.handle_key_event(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.selected, 0);
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_visible_quotes_group_filter() {
        let mut app = test_app();
        app.groups = vec!["All".to_string(), "just_apple".to_string()];
        app.group_symbols = vec![
            vec!["AAPL".to_string(), "GOOGL".to_string()],
            vec!["AAPL".to_string()],
        ];
        // All group
        assert_eq!(app.visible_quotes().len(), 2);

        // Switch to just_apple group
        app.active_group = 1;
        let visible = app.visible_quotes();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].symbol, "AAPL");
    }

    // --- Fundamentals toggle ---

    #[test]
    fn test_toggle_fundamentals() {
        let mut app = test_app();
        assert!(!app.show_fundamentals);
        app.handle_key_event(KeyCode::Char('f'), KeyModifiers::NONE);
        assert!(app.show_fundamentals);
        app.handle_key_event(KeyCode::Char('f'), KeyModifiers::NONE);
        assert!(!app.show_fundamentals);
    }

    // --- Secure mode restrictions ---

    #[test]
    fn test_secure_mode_blocks_toggles() {
        let mut app = test_app();
        app.secure_mode = true;
        app.handle_key_event(KeyCode::Char('h'), KeyModifiers::NONE);
        assert!(!app.show_help, "secure mode should block help toggle");
        app.handle_key_event(KeyCode::Char('H'), KeyModifiers::NONE);
        assert!(!app.show_holdings, "secure mode should block holdings toggle");
        app.handle_key_event(KeyCode::Char('f'), KeyModifiers::NONE);
        assert!(!app.show_fundamentals, "secure mode should block fundamentals toggle");
    }

    // --- Mock QuoteProvider tests ---

    struct MockProvider {
        quotes: Vec<Quote>,
    }

    #[async_trait::async_trait]
    impl crate::models::QuoteProvider for MockProvider {
        async fn get_quotes(&self, _symbols: &[String]) -> anyhow::Result<Vec<Quote>> {
            Ok(self.quotes.clone())
        }
    }

    #[tokio::test]
    async fn test_with_provider_mock() {
        let mock = MockProvider {
            quotes: vec![Quote {
                symbol: "TEST".into(),
                name: "Test Corp".into(),
                price: 42.0,
                change: 2.0,
                change_percent: 5.0,
                ..Quote::default()
            }],
        };

        let args = Args::parse_from(["stonktop", "-s", "TEST", "-b", "-n", "1"]);
        let config = Config::default();
        let mut app = App::with_provider(&args, &config, Box::new(mock)).unwrap();

        app.refresh().await.unwrap();

        assert_eq!(app.quotes.len(), 1);
        assert_eq!(app.quotes[0].symbol, "TEST");
        assert!((app.quotes[0].price - 42.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_refresh_calls_check_alerts() {
        let mock = MockProvider {
            quotes: vec![Quote {
                symbol: "AAPL".into(),
                name: "Apple".into(),
                price: 200.0,
                ..Quote::default()
            }],
        };

        let args = Args::parse_from(["stonktop", "-s", "AAPL", "-b", "-n", "1"]);
        let config = Config::default();
        let mut app = App::with_provider(&args, &config, Box::new(mock)).unwrap();
        app.alerts = vec![Alert {
            symbol: "AAPL".to_string(),
            above: Some(190.0),
            below: None,
        }];

        app.refresh().await.unwrap();

        assert_eq!(app.triggered_alerts.len(), 1, "alert should fire after refresh");
    }

    // --- Save watchlist tests ---

    #[test]
    fn test_save_watchlist_no_config_path() {
        let mut app = test_app();
        app.config_path = None;
        // Should return Ok without doing anything
        assert!(app.save_watchlist().is_ok());
    }

    #[test]
    fn test_save_watchlist_nonexistent_path() {
        let mut app = test_app();
        app.config_path = Some(std::path::PathBuf::from("/tmp/nonexistent_stonktop_test/config.toml"));
        // Should return Ok because path doesn't exist (skip save)
        assert!(app.save_watchlist().is_ok());
    }
}
