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
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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

    // Handle --init flag
    if args.init {
        let path = Config::default_config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        if path.exists() && !args.force {
            eprintln!("Config file already exists: {}", path.display());
            eprintln!("Use --force to overwrite.");
            std::process::exit(1);
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, config::sample_config())?;
        println!("Config file written to: {}", path.display());
        std::process::exit(0);
    }

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
        run_batch(&mut app, &args.format).await
    } else {
        run_interactive(&mut app).await
    }
}

/// Run in batch mode (non-interactive, like top -b).
async fn run_batch(app: &mut App, format: &cli::OutputFormat) -> Result<()> {
    loop {
        app.refresh().await?;
        ui::render_batch(app, format);

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

    // Ensure terminal is restored on panic so the shell isn't left in raw mode.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(stdout, crossterm::cursor::Show);
        original_hook(info);
    }));

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
                    app.handle_key_event(key.code, key.modifiers);
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
