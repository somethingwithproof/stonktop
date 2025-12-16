//! Stonktop - A top-like terminal UI for stock and crypto prices.
//!
//! "Stonks only go up" - Famous last words
//!
//! A terminal-based stock and cryptocurrency price monitor that brings
//! the thrill of watching your portfolio fluctuate directly to your
//! command line. Now you can lose money AND look like a hacker!

mod api;
mod app;
mod cli;
mod config;
mod models;
mod ui;

use anyhow::Result;
use app::App;
use cli::Args;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse_args();

    // Load configuration
    let config = if let Some(ref path) = args.config {
        Config::load(path)?
    } else {
        Config::load_or_default()
    };

    // Create application state
    let mut app = App::new(&args, &config)?;

    // Check if we have any symbols to watch
    if app.symbols.is_empty() {
        eprintln!("Error: No symbols to watch.");
        eprintln!("Provide symbols via -s flag or config file.");
        eprintln!();
        eprintln!("Example: stonktop -s AAPL,GOOGL,BTC-USD");
        eprintln!();
        eprintln!(
            "Or create a config file at {:?}",
            Config::default_config_path()
        );
        eprintln!();
        eprintln!("Sample config:");
        eprintln!("{}", config::sample_config());
        std::process::exit(1);
    }

    // Run in batch mode or interactive mode
    if app.batch_mode {
        run_batch(&mut app).await
    } else {
        run_interactive(&mut app).await
    }
}

/// Run in batch mode (non-interactive, like top -b).
async fn run_batch(app: &mut App) -> Result<()> {
    loop {
        app.refresh().await?;
        ui::render_batch(app);

        if app.should_quit() {
            break;
        }

        tokio::time::sleep(app.refresh_interval).await;
    }

    Ok(())
}

/// Run in interactive mode with TUI.
async fn run_interactive(app: &mut App) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial fetch
    app.refresh().await?;

    // Main loop
    let result = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application loop.
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let tick_rate = Duration::from_millis(100);

    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle events with timeout
        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                // Skip if secure mode and it's a modifying command
                if app.secure_mode {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                        KeyCode::Up | KeyCode::Char('k') => app.select_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.select_down(),
                        _ => {}
                    }
                } else {
                    handle_key_event(app, key.code, key.modifiers);
                }
            }
        }

        // Check if we should quit
        if app.should_quit() {
            break;
        }

        // Refresh data if needed
        if app.needs_refresh() {
            app.refresh().await?;
        }
    }

    Ok(())
}

/// Handle keyboard input.
fn handle_key_event(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    // Close help overlay on any key
    if app.show_help {
        app.show_help = false;
        return;
    }

    // Clear error on any key
    if app.error.is_some() {
        app.error = None;
        return;
    }

    match code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => app.quit(),

        // Navigation
        KeyCode::Up | KeyCode::Char('k') => app.select_up(),
        KeyCode::Down | KeyCode::Char('j') => app.select_down(),
        KeyCode::Home | KeyCode::Char('g') => app.select_top(),
        KeyCode::End | KeyCode::Char('G') => app.select_bottom(),
        KeyCode::PageUp => {
            for _ in 0..10 {
                app.select_up();
            }
        }
        KeyCode::PageDown => {
            for _ in 0..10 {
                app.select_down();
            }
        }

        // Sorting
        KeyCode::Char('s') => app.next_sort_order(),
        KeyCode::Char('r') => app.toggle_sort_direction(),
        KeyCode::Char('1') => app.set_sort_order(models::SortOrder::Symbol),
        KeyCode::Char('2') => app.set_sort_order(models::SortOrder::Name),
        KeyCode::Char('3') => app.set_sort_order(models::SortOrder::Price),
        KeyCode::Char('4') => app.set_sort_order(models::SortOrder::Change),
        KeyCode::Char('5') => app.set_sort_order(models::SortOrder::ChangePercent),
        KeyCode::Char('6') => app.set_sort_order(models::SortOrder::Volume),
        KeyCode::Char('7') => app.set_sort_order(models::SortOrder::MarketCap),

        // Display toggles
        KeyCode::Char('H') => app.toggle_holdings(),
        KeyCode::Char('f') => app.toggle_fundamentals(),
        KeyCode::Char('h') | KeyCode::Char('?') => app.toggle_help(),

        // Refresh
        KeyCode::Char(' ') | KeyCode::Char('R') => {
            app.last_refresh = None; // Force refresh on next tick
        }

        // Groups
        KeyCode::Tab => {
            if !app.groups.is_empty() {
                app.active_group = (app.active_group + 1) % app.groups.len();
            }
        }

        _ => {}
    }
}
