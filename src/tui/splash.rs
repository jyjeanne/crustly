//! Splash Screen
//!
//! Startup welcome screen with logo and project information.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the splash screen
pub fn render_splash(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(20),
            Constraint::Min(0),
        ])
        .split(area);

    // Center horizontally
    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(80),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    render_splash_content(f, center_chunks[1], provider_name, model_name);
}

fn render_splash_content(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str) {
    let version = env!("CARGO_PKG_VERSION");

    let splash_text = vec![
        Line::from(""),
        Line::from(""),
        // Clean ASCII Text Logo
        Line::from(vec![
            Span::raw("                "),
            Span::styled(
                "   ___             _   _",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                "),
            Span::styled(
                "  / __|_ _ _  _ __| |_| |_  _",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                "),
            Span::styled(
                " | (__| '_| || (_-<  _| | || |",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                "),
            Span::styled(
                r"  \___|_|  \_,_/__/\__|_|\_, |",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                "),
            Span::styled(
                "                         |__/",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("                        "),
            Span::styled(
                "🥐 Flaky & Fast",
                Style::default()
                    .fg(Color::Rgb(184, 134, 11))
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("                    "),
            Span::styled(
                "by Jeremy JEANNE",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        // Project name and version
        Line::from(vec![
            Span::styled("              ╭─── ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "🥐 Crustly",
                Style::default()
                    .fg(Color::Rgb(218, 165, 32))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" v{} ", version),
                Style::default()
                    .fg(Color::Rgb(184, 134, 11))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("───╮", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        // Model and details
        Line::from(vec![
            Span::raw("                    "),
            Span::styled("Model: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                model_name,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                    "),
            Span::styled("Provider: ", Style::default().fg(Color::DarkGray)),
            Span::styled(provider_name, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "              High-performance terminal AI assistant",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::styled(
            "                   Press any key to continue...",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let splash = Paragraph::new(splash_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(splash, area);
}
