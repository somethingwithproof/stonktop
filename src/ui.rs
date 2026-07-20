//! Terminal user interface with ratatui.
//!
//! Making financial data look pretty since 2024.
//! (The data itself? Still ugly. That's not our fault.)

use crate::app::App;
use crate::models::SortOrder;
use num_format::{Locale, ToFormattedString};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Sparkline, Table, TableState, Wrap},
    Frame,
};

/// Colors for the UI.
pub struct UiColors {
    pub gain: Color,
    pub loss: Color,
    pub neutral: Color,
    pub header_bg: Color,
    pub selected_bg: Color,
    pub border: Color,
}

impl Default for UiColors {
    fn default() -> Self {
        Self {
            gain: Color::Green,
            loss: Color::Red,
            neutral: Color::White,
            header_bg: Color::DarkGray,
            selected_bg: Color::Rgb(40, 40, 60),
            border: Color::DarkGray,
        }
    }
}

impl UiColors {
    /// Create colors appropriate for the given color mode and config.
    pub fn for_app(app: &App) -> Self {
        match app.ui.color_mode {
            crate::cli::ColorMode::Never => Self {
                gain: Color::Reset,
                loss: Color::Reset,
                neutral: Color::Reset,
                header_bg: Color::Reset,
                selected_bg: Color::Reset,
                border: Color::Reset,
            },
            _ => Self::from_config(&app.ui.color_config),
        }
    }

    /// Parse a hex color string like "#00ff00" into a ratatui Color.
    /// Returns None if the string is not a valid 6-digit hex color.
    pub fn parse_hex(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    }

    /// Create colors from a ColorConfig, falling back to defaults.
    pub fn from_config(config: &crate::config::ColorConfig) -> Self {
        let defaults = Self::default();
        Self {
            gain: Self::parse_hex(&config.gain).unwrap_or(defaults.gain),
            loss: Self::parse_hex(&config.loss).unwrap_or(defaults.loss),
            neutral: Self::parse_hex(&config.neutral).unwrap_or(defaults.neutral),
            header_bg: Self::parse_hex(&config.header).unwrap_or(defaults.header_bg),
            selected_bg: defaults.selected_bg,
            border: Self::parse_hex(&config.border).unwrap_or(defaults.border),
        }
    }
}

/// Render the main UI.
pub fn render(frame: &mut Frame, app: &App) {
    let colors = UiColors::for_app(app);

    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main table
            Constraint::Length(1), // Footer
        ])
        .split(frame.area());

    // Render header
    render_header(frame, app, chunks[0], &colors);

    // Render main table (with optional sparkline area)
    if app.ui.show_sparklines && !app.ui.show_holdings {
        let table_spark = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(60), Constraint::Length(22)])
            .split(chunks[1]);
        render_quotes_table(frame, app, table_spark[0], &colors);
        render_sparklines(frame, app, table_spark[1], &colors);
    } else if app.ui.show_holdings {
        render_holdings_table(frame, app, chunks[1], &colors);
    } else {
        render_quotes_table(frame, app, chunks[1], &colors);
    }

    // Render footer
    render_footer(frame, app, chunks[2], &colors);

    // Render help overlay if active
    if app.ui.show_help {
        render_help_overlay(frame, &colors);
    }

    // Render alerts if any triggered
    if !app.domain.triggered_alerts.is_empty() {
        render_alerts(frame, app, &colors);
    }

    // Render error if present
    if let Some(ref error) = app.error {
        render_error(frame, error, &colors);
    }

    // Render detail popup if active
    if app.ui.show_detail {
        render_detail(frame, app, &colors);
    }
}

/// Render the header with summary information.
fn render_header(frame: &mut Frame, app: &App, area: Rect, colors: &UiColors) {
    let visible = app.visible_quotes();
    let gains = visible.iter().filter(|q| q.change_percent > 0.0).count();
    let losses = visible.iter().filter(|q| q.change_percent < 0.0).count();
    let unchanged = visible.len() - gains - losses;
    let markets_open = app.any_markets_open();

    let header_text = if app.ui.show_holdings {
        let total_value = app.total_portfolio_value();
        let total_pnl = app.total_portfolio_pnl();
        let today_change = app.today_portfolio_change();
        let pnl_pct = if app.total_portfolio_cost() > 0.0 {
            (total_pnl / app.total_portfolio_cost()) * 100.0
        } else {
            0.0
        };

        vec![
            Line::from(vec![
                Span::styled(
                    "STONKTOP ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("- Portfolio View"),
            ]),
            Line::from(vec![
                Span::raw(format!("Value: ${:.2}  ", total_value)),
                Span::styled(
                    format!("P/L: {:+.2} ({:+.2}%)  ", total_pnl, pnl_pct),
                    Style::default().fg(if total_pnl >= 0.0 {
                        colors.gain
                    } else {
                        colors.loss
                    }),
                ),
                Span::styled(
                    format!("Today: {:+.2}", today_change),
                    Style::default().fg(if today_change >= 0.0 {
                        colors.gain
                    } else {
                        colors.loss
                    }),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled(
                    "STONKTOP ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    "- {} symbols [{}]",
                    visible.len(),
                    app.active_group_name()
                )),
            ]),
            Line::from(vec![
                Span::styled(format!("{} ", gains), Style::default().fg(colors.gain)),
                Span::raw("up  "),
                Span::styled(format!("{} ", losses), Style::default().fg(colors.loss)),
                Span::raw("down  "),
                Span::raw(format!("{} unchanged  ", unchanged)),
                Span::raw(format!("Updated: {} ", app.time_since_refresh())),
                Span::styled(
                    if markets_open { "[open]" } else { "[closed]" },
                    Style::default().fg(if markets_open {
                        colors.gain
                    } else {
                        colors.loss
                    }),
                ),
            ]),
        ]
    };

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(colors.border)),
    );

    frame.render_widget(header, area);
}

/// Render the quotes table.
fn render_quotes_table(frame: &mut Frame, app: &App, area: Rect, colors: &UiColors) {
    let mut base_headers: Vec<(&str, SortOrder)> = vec![
        ("SYMBOL", SortOrder::Symbol),
        ("NAME", SortOrder::Name),
        ("PRICE", SortOrder::Price),
        ("CHANGE", SortOrder::Change),
        ("CHG%", SortOrder::ChangePercent),
        ("VOLUME", SortOrder::Volume),
        ("MKT CAP", SortOrder::MarketCap),
    ];

    // Fundamentals mode adds price range columns
    if app.ui.show_fundamentals {
        base_headers.push(("OPEN", SortOrder::Price));
        base_headers.push(("HIGH", SortOrder::Price));
        base_headers.push(("LOW", SortOrder::Price));
        base_headers.push(("52W H", SortOrder::Price));
        base_headers.push(("52W L", SortOrder::Price));
    }

    // Verbose mode adds extra columns for the truly data-hungry
    if app.ui.verbose {
        base_headers.push(("EXCHANGE", SortOrder::Symbol)); // no dedicated sort
        base_headers.push(("CCY", SortOrder::Symbol));
        base_headers.push(("TYPE", SortOrder::Symbol));
    }

    let header_cells = base_headers.iter().map(|(name, order)| {
        let style = if app.domain.sort_order == *order && *name == order.header() {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let indicator = if app.domain.sort_order == *order && *name == order.header() {
            match app.domain.sort_direction {
                crate::models::SortDirection::Ascending => " ▲",
                crate::models::SortDirection::Descending => " ▼",
            }
        } else {
            ""
        };

        Cell::from(format!("{}{}", name, indicator)).style(style)
    });

    let header = Row::new(header_cells)
        .style(Style::default().bg(colors.header_bg))
        .height(1);

    let filtered = app.visible_quotes();
    let visible_quotes: Vec<_> = filtered
        .iter()
        .enumerate()
        .skip(app.ui.scroll_offset)
        .collect();

    let rows = visible_quotes.iter().map(|(i, quote)| {
        let is_selected = *i == app.ui.selected;
        let change_color = if quote.change_percent > 0.0 {
            colors.gain
        } else if quote.change_percent < 0.0 {
            colors.loss
        } else {
            colors.neutral
        };

        let row_style = if is_selected {
            Style::default().bg(colors.selected_bg)
        } else {
            Style::default()
        };

        let mut cells = vec![
            Cell::from(quote.symbol.clone()),
            Cell::from(truncate_string(&quote.name, 20)),
            Cell::from(format_price(quote.price)),
            Cell::from(format!("{:+.2}", quote.change)).style(Style::default().fg(change_color)),
            Cell::from(format!("{:+.2}%", quote.change_percent))
                .style(Style::default().fg(change_color)),
            Cell::from(format_volume(quote.volume)),
            Cell::from(format_market_cap(quote.market_cap)),
        ];

        if app.ui.show_fundamentals {
            cells.push(Cell::from(format_price(quote.open)));
            cells.push(Cell::from(format_price(quote.day_high)));
            cells.push(Cell::from(format_price(quote.day_low)));
            cells.push(Cell::from(format_price(quote.year_high)));
            cells.push(Cell::from(format_price(quote.year_low)));
        }

        if app.ui.verbose {
            cells.push(Cell::from(truncate_string(&quote.exchange, 10)));
            cells.push(Cell::from(quote.currency.clone()));
            cells.push(Cell::from(format!("{}", quote.quote_type)));
        }

        Row::new(cells).style(row_style)
    });

    let mut widths = vec![
        Constraint::Length(10),
        Constraint::Length(22),
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(12),
        Constraint::Length(12),
    ];

    if app.ui.show_fundamentals {
        widths.push(Constraint::Length(12)); // OPEN
        widths.push(Constraint::Length(12)); // HIGH
        widths.push(Constraint::Length(12)); // LOW
        widths.push(Constraint::Length(12)); // 52W H
        widths.push(Constraint::Length(12)); // 52W L
    }

    if app.ui.verbose {
        widths.push(Constraint::Length(12));
        widths.push(Constraint::Length(5));
        widths.push(Constraint::Length(8));
    }

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::NONE))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let adjusted_selected = app.ui.selected.saturating_sub(app.ui.scroll_offset);
    let mut state = TableState::default();
    state.select(Some(adjusted_selected));

    frame.render_stateful_widget(table, area, &mut state);
}

/// Render the holdings/portfolio table.
fn render_holdings_table(frame: &mut Frame, app: &App, area: Rect, colors: &UiColors) {
    let header_cells = [
        "SYMBOL", "NAME", "PRICE", "QTY", "VALUE", "COST", "P/L", "P/L%", "TODAY",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));

    let header = Row::new(header_cells)
        .style(Style::default().bg(colors.header_bg))
        .height(1);

    let visible = app.visible_quotes();
    let holdings_data: Vec<_> = visible
        .iter()
        .filter_map(|quote| {
            let holding = app.domain.holdings.get(&quote.symbol)?;
            Some((*quote, holding))
        })
        .collect();

    let rows = holdings_data
        .iter()
        .enumerate()
        .skip(app.ui.scroll_offset)
        .map(|(display_idx, (quote, holding))| {
            let is_selected = display_idx == app.ui.selected;

            let value = holding.current_value(quote.price);
            let cost = holding.total_cost();
            let pnl = holding.profit_loss(quote.price);
            let pnl_pct = holding.profit_loss_percent(quote.price);
            let today = holding.quantity * quote.change;

            let pnl_color = if pnl >= 0.0 { colors.gain } else { colors.loss };
            let today_color = if today >= 0.0 {
                colors.gain
            } else {
                colors.loss
            };

            let row_style = if is_selected {
                Style::default().bg(colors.selected_bg)
            } else {
                Style::default()
            };

            let cells = vec![
                Cell::from(quote.symbol.clone()),
                Cell::from(truncate_string(&quote.name, 15)),
                Cell::from(format_price(quote.price)),
                Cell::from(format!("{:.4}", holding.quantity)),
                Cell::from(format!("${:.2}", value)),
                Cell::from(format!("${:.2}", cost)),
                Cell::from(format!("{:+.2}", pnl)).style(Style::default().fg(pnl_color)),
                Cell::from(format!("{:+.2}%", pnl_pct)).style(Style::default().fg(pnl_color)),
                Cell::from(format!("{:+.2}", today)).style(Style::default().fg(today_color)),
            ];

            Row::new(cells).style(row_style)
        });

    let widths = [
        Constraint::Length(10),
        Constraint::Length(17),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::NONE))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let adjusted_selected = app.ui.selected.saturating_sub(app.ui.scroll_offset);
    let mut state = TableState::default();
    state.select(Some(adjusted_selected));
    frame.render_stateful_widget(table, area, &mut state);
}

/// Render the footer with keybindings.
fn render_footer(frame: &mut Frame, app: &App, area: Rect, colors: &UiColors) {
    // Input mode gets a special prompt
    match app.ui.input_mode {
        crate::app::InputMode::AddSymbol => {
            let input_line = Line::from(vec![
                Span::styled(" Add symbol: ", Style::default().fg(Color::Yellow)),
                Span::raw(&app.ui.input_buffer),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ]);
            let footer_widget =
                Paragraph::new(input_line).style(Style::default().bg(colors.header_bg));
            frame.render_widget(footer_widget, area);
            return;
        }
        crate::app::InputMode::Search => {
            let input_line = Line::from(vec![
                Span::styled(" /", Style::default().fg(Color::Yellow)),
                Span::raw(&app.ui.search_filter),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
                Span::raw(format!("  ({} matches)", app.visible_quotes().len())),
            ]);
            let footer_widget =
                Paragraph::new(input_line).style(Style::default().bg(colors.header_bg));
            frame.render_widget(footer_widget, area);
            return;
        }
        crate::app::InputMode::Normal => {}
    }

    let mode = if app.ui.show_holdings {
        "Holdings"
    } else {
        "Quotes"
    };
    let sort_info = format!(
        "{} {}",
        app.domain.sort_order.header(),
        match app.domain.sort_direction {
            crate::models::SortDirection::Ascending => "▲",
            crate::models::SortDirection::Descending => "▼",
        }
    );

    let footer = Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Yellow)),
        Span::raw(":quit "),
        Span::styled("h", Style::default().fg(Color::Yellow)),
        Span::raw(":help "),
        Span::styled("s", Style::default().fg(Color::Yellow)),
        Span::raw(":sort "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(":reverse "),
        Span::styled("H", Style::default().fg(Color::Yellow)),
        Span::raw(":holdings "),
        Span::styled("f", Style::default().fg(Color::Yellow)),
        Span::raw(":fundamentals "),
        Span::raw(format!(
            "| {} | {} | Iter: {}",
            mode, sort_info, app.iteration
        )),
    ]);

    let footer_widget = Paragraph::new(footer).style(Style::default().bg(colors.header_bg));

    frame.render_widget(footer_widget, area);
}

/// Render help overlay.
fn render_help_overlay(frame: &mut Frame, colors: &UiColors) {
    let area = centered_rect(60, 70, frame.area());

    let help_text = vec![
        Line::from(Span::styled(
            "STONKTOP HELP",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/k       Move up"),
        Line::from("  ↓/j       Move down"),
        Line::from("  g/Home    Go to top"),
        Line::from("  G/End     Go to bottom"),
        Line::from("  PgUp      Page up"),
        Line::from("  PgDn      Page down"),
        Line::from(""),
        Line::from("Sorting:"),
        Line::from("  s         Cycle sort field"),
        Line::from("  r         Reverse sort order"),
        Line::from("  1-7       Sort by column"),
        Line::from(""),
        Line::from("Display:"),
        Line::from("  H         Toggle holdings view"),
        Line::from("  f         Toggle fundamentals"),
        Line::from("  S         Toggle sparklines"),
        Line::from("  Tab       Cycle groups"),
        Line::from(""),
        Line::from("Symbols:"),
        Line::from("  a         Add symbol"),
        Line::from("  d         Remove symbol"),
        Line::from("  /         Search/filter"),
        Line::from("  Enter     Quote detail"),
        Line::from("  e         Export watchlist"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  Space/R   Force refresh"),
        Line::from("  q/Esc     Quit"),
        Line::from("  h/?       Toggle help"),
        Line::from(""),
        Line::from("Mouse: click to select, scroll to navigate"),
        Line::from(""),
        Line::from("Press any key to close"),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, area);
    frame.render_widget(help, area);
}

/// Render error message.
fn render_error(frame: &mut Frame, error: &str, colors: &UiColors) {
    let area = centered_rect(50, 20, frame.area());

    let error_widget = Paragraph::new(error)
        .block(
            Block::default()
                .title(" Error ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.loss)),
        )
        .style(Style::default().fg(colors.loss))
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(error_widget, area);
}

/// Create a centered rectangle.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let px = percent_x.min(100);
    let py = percent_y.min(100);
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Format price with appropriate precision.
/// Penny stocks get more decimals because every fraction of a cent matters
/// when you're hoping for that 10,000% gain.
pub(crate) fn format_price(price: f64) -> String {
    if price == 0.0 {
        "$0.00".to_string()
    } else if price.abs() >= 1.0 {
        format!("${:.2}", price)
    } else {
        format!("${:.6}", price)
    }
}

/// Format volume with suffixes.
pub(crate) fn format_volume(volume: u64) -> String {
    if volume >= 1_000_000_000 {
        format!("{:.2}B", volume as f64 / 1_000_000_000.0)
    } else if volume >= 1_000_000 {
        format!("{:.2}M", volume as f64 / 1_000_000.0)
    } else if volume >= 1_000 {
        format!("{:.2}K", volume as f64 / 1_000.0)
    } else {
        volume.to_formatted_string(&Locale::en)
    }
}

/// Format market cap with suffixes.
pub(crate) fn format_market_cap(market_cap: Option<u64>) -> String {
    match market_cap {
        Some(cap) if cap >= 1_000_000_000_000 => {
            format!("${:.2}T", cap as f64 / 1_000_000_000_000.0)
        }
        Some(cap) if cap >= 1_000_000_000 => format!("${:.2}B", cap as f64 / 1_000_000_000.0),
        Some(cap) if cap >= 1_000_000 => format!("${:.2}M", cap as f64 / 1_000_000.0),
        Some(cap) => format!("${}", cap.to_formatted_string(&Locale::en)),
        None => "-".to_string(),
    }
}

/// Truncate string to max length.
pub(crate) fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        let mut end = max_len.saturating_sub(3);
        while !s.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

/// Build a visual range bar showing where a value sits between low and high.
/// The ● marker is placed between ├ and ┤ without overwriting the endpoints.
fn range_bar(low: f64, high: f64, current: f64, width: usize) -> String {
    if high <= low || width < 3 {
        return format!("{:.2} — {:.2}", low, high);
    }
    let pos = ((current - low) / (high - low)).clamp(0.0, 1.0);
    let inner = width - 2;
    let idx = if inner <= 1 {
        1
    } else {
        1 + (pos * (inner - 1) as f64).round() as usize
    };
    let mut bar: Vec<char> = vec!['─'; width];
    bar[0] = '├';
    bar[width - 1] = '┤';
    bar[idx] = '●';
    bar.iter().collect()
}

/// Render detail popup for the selected quote.
fn render_detail(frame: &mut Frame, app: &App, colors: &UiColors) {
    if let Some(quote) = app.selected_quote() {
        let area = centered_rect(65, 75, frame.area());

        let fmt_or_na = |v: f64| -> String {
            if v == 0.0 {
                "N/A".to_string()
            } else {
                format_price(v)
            }
        };

        let change_color = if quote.change_percent >= 0.0 {
            colors.gain
        } else {
            colors.loss
        };

        let day_range_bar = if quote.day_low > 0.0 && quote.day_high > 0.0 {
            range_bar(quote.day_low, quote.day_high, quote.price, 20)
        } else {
            "N/A".to_string()
        };

        let year_range_bar = if quote.year_low > 0.0 && quote.year_high > 0.0 {
            range_bar(quote.year_low, quote.year_high, quote.price, 20)
        } else {
            "N/A".to_string()
        };

        let mut detail_text = vec![
            Line::from(Span::styled(
                format!(" {} - {} ", quote.symbol, quote.name),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw(format!("  Price:          {}", format_price(quote.price))),
                Span::raw("  "),
                Span::styled(
                    format!("{:+.2} ({:+.2}%)", quote.change, quote.change_percent),
                    Style::default().fg(change_color),
                ),
            ]),
            Line::from(format!(
                "  Prev Close:     {}",
                fmt_or_na(quote.previous_close)
            )),
            Line::from(format!("  Open:           {}", fmt_or_na(quote.open))),
            Line::from(""),
            Line::from(format!(
                "  Day Range:      {} - {}",
                fmt_or_na(quote.day_low),
                fmt_or_na(quote.day_high)
            )),
            Line::from(format!("                  {}", day_range_bar)),
            Line::from(""),
            Line::from(format!(
                "  52w Range:      {} - {}",
                fmt_or_na(quote.year_low),
                fmt_or_na(quote.year_high)
            )),
            Line::from(format!("                  {}", year_range_bar)),
            Line::from(""),
            Line::from(format!("  Volume:         {}", format_volume(quote.volume))),
            Line::from(format!(
                "  Avg Volume:     {}",
                format_volume(quote.avg_volume)
            )),
            Line::from(format!(
                "  Market Cap:     {}",
                format_market_cap(quote.market_cap)
            )),
            Line::from(format!("  Exchange:       {}", quote.exchange)),
            Line::from(format!("  Currency:       {}", quote.currency)),
            Line::from(format!("  Type:           {}", quote.quote_type)),
            Line::from(format!("  Market:         {}", quote.market_state)),
            Line::from(format!(
                "  Timestamp:      {}",
                quote.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            )),
        ];

        // Sparkline history if available
        if let Some(history) = app.domain.price_history.get(&quote.symbol) {
            if history.len() > 1 {
                if let Some((min, max)) = history.min_max() {
                    detail_text.push(Line::from(""));
                    detail_text.push(Line::from(format!(
                        "  Price History:  {} pts ({:.2} - {:.2})",
                        history.len(),
                        min,
                        max
                    )));
                }
            }
        }

        // Currency conversion info
        if app.domain.currency_convert && quote.currency != app.domain.display_currency {
            let converted = app.convert_price(quote.price, &quote.currency);
            detail_text.push(Line::from(format!(
                "  Converted:      {}{:.2} {}",
                app.currency_symbol(),
                converted,
                app.domain.display_currency
            )));
        }

        detail_text.push(Line::from(""));
        detail_text.push(Line::from("  Press any key to close"));

        let detail = Paragraph::new(detail_text)
            .block(
                Block::default()
                    .title(" Quote Detail ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border)),
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(Clear, area);
        frame.render_widget(detail, area);
    }
}

/// Render triggered alerts as a notification bar.
fn render_alerts(frame: &mut Frame, app: &App, colors: &UiColors) {
    let area = centered_rect(50, 20, frame.area());
    let text: Vec<Line> = app
        .domain
        .triggered_alerts
        .iter()
        .map(|(_, msg)| {
            Line::from(Span::styled(
                msg.as_str(),
                Style::default().fg(Color::Yellow),
            ))
        })
        .collect();

    let alert_widget = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Price Alerts ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(colors.gain))
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(alert_widget, area);
}

/// Render sparkline charts sidebar.
fn render_sparklines(frame: &mut Frame, app: &App, area: Rect, colors: &UiColors) {
    let filtered = app.visible_quotes();
    let visible: Vec<_> = filtered.iter().skip(app.ui.scroll_offset).collect();

    // 1 row for header + 1 row per quote
    let mut constraints = vec![Constraint::Length(1)];
    for _ in &visible {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    // Header
    let header = Paragraph::new(Span::styled(
        " TREND",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .style(Style::default().bg(colors.header_bg));
    frame.render_widget(header, rows[0]);

    for (i, quote) in visible.iter().enumerate() {
        if i + 1 >= rows.len() - 1 {
            break;
        }
        let row_area = rows[i + 1];
        if let Some(history) = app.domain.price_history.get(&quote.symbol) {
            if history.len() > 1 {
                let data = history.sparkline_data();
                let color = if quote.change_percent >= 0.0 {
                    colors.gain
                } else {
                    colors.loss
                };
                let spark = Sparkline::default()
                    .data(data)
                    .style(Style::default().fg(color));
                frame.render_widget(spark, row_area);
            }
        }
    }
}

/// Render batch mode output (non-interactive).
pub fn render_batch(app: &App, format: &crate::cli::OutputFormat) {
    match format {
        crate::cli::OutputFormat::Table => render_batch_table(app),
        crate::cli::OutputFormat::Json => render_batch_json(app),
        crate::cli::OutputFormat::Csv => render_batch_csv(app),
    }
}

/// Batch output as a classic table.
fn render_batch_table(app: &App) {
    use chrono::Local;

    println!(
        "\n=== STONKTOP {} ===",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    if app.ui.show_holdings {
        println!(
            "{:<10} {:<15} {:>10} {:>10} {:>12} {:>12} {:>10} {:>10}",
            "SYMBOL", "NAME", "PRICE", "QTY", "VALUE", "COST", "P/L", "P/L%"
        );
        println!("{}", "-".repeat(100));

        for quote in &app.domain.quotes {
            if let Some(holding) = app.domain.holdings.get(&quote.symbol) {
                let value = holding.current_value(quote.price);
                let cost = holding.total_cost();
                let pnl = holding.profit_loss(quote.price);
                let pnl_pct = holding.profit_loss_percent(quote.price);

                println!(
                    "{:<10} {:<15} {:>10.2} {:>10.4} {:>12.2} {:>12.2} {:>+10.2} {:>+9.2}%",
                    quote.symbol,
                    truncate_string(&quote.name, 15),
                    quote.price,
                    holding.quantity,
                    value,
                    cost,
                    pnl,
                    pnl_pct
                );
            }
        }
    } else {
        println!(
            "{:<10} {:<20} {:>12} {:>10} {:>10} {:>12} {:>12}",
            "SYMBOL", "NAME", "PRICE", "CHANGE", "CHG%", "VOLUME", "MKT CAP"
        );
        println!("{}", "-".repeat(90));

        for quote in &app.domain.quotes {
            println!(
                "{:<10} {:<20} {:>12} {:>+10.2} {:>+9.2}% {:>12} {:>12}",
                quote.symbol,
                truncate_string(&quote.name, 20),
                format_price(quote.price),
                quote.change,
                quote.change_percent,
                format_volume(quote.volume),
                format_market_cap(quote.market_cap)
            );
        }
    }

    println!();
}

/// Batch output as JSON — for the pipeline-minded.
fn render_batch_json(app: &App) {
    match serde_json::to_string_pretty(&app.domain.quotes) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("JSON serialization error: {}", e),
    }
}

/// RFC 4180: escape a field that may contain commas, quotes, or newlines.
fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Batch output as CSV — for spreadsheet warriors everywhere.
fn render_batch_csv(app: &App) {
    println!("symbol,name,price,change,change_percent,volume,market_cap");
    for quote in &app.domain.quotes {
        println!(
            "{},{},{:.2},{:+.2},{:+.2},{},{}",
            csv_escape(&quote.symbol),
            csv_escape(&quote.name),
            quote.price,
            quote.change,
            quote.change_percent,
            quote.volume,
            quote.market_cap.map_or("-".to_string(), |c| c.to_string())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- format_price tests ---

    #[test]
    fn test_format_price_normal() {
        assert_eq!(format_price(195.89), "$195.89");
        assert_eq!(format_price(1.00), "$1.00");
        assert_eq!(format_price(1000.50), "$1000.50");
    }

    #[test]
    fn test_format_price_penny() {
        assert_eq!(format_price(0.001234), "$0.001234");
        assert_eq!(format_price(0.99), "$0.990000");
    }

    #[test]
    fn test_format_price_zero() {
        assert_eq!(format_price(0.0), "$0.00");
    }

    #[test]
    fn test_format_price_negative() {
        assert_eq!(format_price(-5.50), "$-5.50");
        assert_eq!(format_price(-0.001234), "$-0.001234");
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(csv_escape("simple"), "simple");
        assert_eq!(csv_escape("has,comma"), "\"has,comma\"");
        assert_eq!(csv_escape("has\"quote"), "\"has\"\"quote\"");
        assert_eq!(csv_escape("has\nnewline"), "\"has\nnewline\"");
        assert_eq!(csv_escape("has\rcarriage"), "\"has\rcarriage\"");
    }

    // --- parse_hex tests ---

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(UiColors::parse_hex("#00ff00"), Some(Color::Rgb(0, 255, 0)));
        assert_eq!(UiColors::parse_hex("#ff0000"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(
            UiColors::parse_hex("1e90ff"),
            Some(Color::Rgb(30, 144, 255))
        );
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert_eq!(UiColors::parse_hex(""), None);
        assert_eq!(UiColors::parse_hex("#fff"), None);
        assert_eq!(UiColors::parse_hex("banana"), None);
        assert_eq!(UiColors::parse_hex("#gggggg"), None);
    }

    // --- format_volume tests ---

    #[test]
    fn test_format_volume_billions() {
        assert_eq!(format_volume(2_000_000_000), "2.00B");
    }

    #[test]
    fn test_format_volume_millions() {
        assert_eq!(format_volume(1_500_000), "1.50M");
    }

    #[test]
    fn test_format_volume_thousands() {
        assert_eq!(format_volume(1_000), "1.00K");
    }

    #[test]
    fn test_format_volume_small() {
        assert_eq!(format_volume(999), "999");
        assert_eq!(format_volume(0), "0");
    }

    // --- format_market_cap tests ---

    #[test]
    fn test_format_market_cap_trillions() {
        assert_eq!(format_market_cap(Some(3_000_000_000_000)), "$3.00T");
    }

    #[test]
    fn test_format_market_cap_billions() {
        assert_eq!(format_market_cap(Some(5_000_000_000)), "$5.00B");
    }

    #[test]
    fn test_format_market_cap_millions() {
        assert_eq!(format_market_cap(Some(5_000_000)), "$5.00M");
    }

    #[test]
    fn test_format_market_cap_small() {
        assert_eq!(format_market_cap(Some(500)), "$500");
    }

    #[test]
    fn test_format_market_cap_none() {
        assert_eq!(format_market_cap(None), "-");
    }

    // --- truncate_string tests ---

    #[test]
    fn test_truncate_within_limit() {
        assert_eq!(truncate_string("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_at_limit() {
        assert_eq!(truncate_string("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_over_limit() {
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }

    #[test]
    fn test_truncate_tiny_max() {
        assert_eq!(truncate_string("hello", 3), "...");
        assert_eq!(truncate_string("hello", 2), "..");
        assert_eq!(truncate_string("hello", 1), ".");
    }

    #[test]
    fn test_truncate_multibyte_utf8() {
        // Should not panic on multi-byte chars
        let s = "héllo wörld";
        let result = truncate_string(s, 6);
        assert!(result.len() <= 9); // byte len, not char len
        assert!(result.ends_with("..."));
    }

    // --- range_bar tests ---

    #[test]
    fn test_range_bar_at_low() {
        let bar = range_bar(100.0, 200.0, 100.0, 10);
        assert!(bar.starts_with("├●"));
        assert!(bar.ends_with('┤'));
    }

    #[test]
    fn test_range_bar_at_high() {
        let bar = range_bar(100.0, 200.0, 200.0, 10);
        assert!(bar.ends_with("●┤"));
        assert!(bar.starts_with('├'));
    }

    #[test]
    fn test_range_bar_at_mid() {
        let bar = range_bar(0.0, 100.0, 50.0, 11);
        assert!(bar.contains('●'));
        assert!(bar.starts_with('├'));
        assert!(bar.ends_with('┤'));
    }

    #[test]
    fn test_range_bar_zero_range() {
        let bar = range_bar(100.0, 100.0, 100.0, 10);
        assert!(bar.contains("100.00"));
    }

    #[test]
    fn test_range_bar_tiny_width() {
        let bar = range_bar(0.0, 100.0, 50.0, 2);
        assert!(bar.contains("0.00"));
    }

    #[test]
    fn test_range_bar_clamped() {
        let bar = range_bar(100.0, 200.0, 250.0, 10);
        assert!(bar.ends_with("●┤"));
        assert!(bar.starts_with('├'));
        let bar2 = range_bar(100.0, 200.0, 50.0, 10);
        assert!(bar2.starts_with("├●"));
        assert!(bar2.ends_with('┤'));
    }

    #[test]
    fn test_range_bar_width_3() {
        let bar = range_bar(0.0, 100.0, 50.0, 3);
        assert_eq!(bar, "├●┤");
    }

    #[test]
    fn test_range_bar_endpoints_preserved() {
        let bar_low = range_bar(0.0, 100.0, 0.0, 10);
        let chars: Vec<char> = bar_low.chars().collect();
        assert_eq!(chars[0], '├');
        assert_eq!(chars[9], '┤');
        assert_eq!(chars[1], '●');

        let bar_high = range_bar(0.0, 100.0, 100.0, 10);
        let chars: Vec<char> = bar_high.chars().collect();
        assert_eq!(chars[0], '├');
        assert_eq!(chars[9], '┤');
        assert_eq!(chars[8], '●');
    }
}
