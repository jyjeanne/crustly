//! TUI Rendering
//!
//! Main rendering logic for the terminal interface.

use super::app::App;
use super::events::AppMode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the entire UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(5), // Input
            Constraint::Length(1), // Status bar
        ])
        .split(f.size());

    // Render components based on mode
    render_header(f, app, chunks[0]);

    match app.mode {
        AppMode::Chat => {
            render_chat(f, app, chunks[1]);
            render_input(f, app, chunks[2]);
        }
        AppMode::Sessions => {
            render_sessions(f, app, chunks[1]);
        }
        AppMode::Help => {
            render_help(f, app, chunks[1]);
        }
        AppMode::Settings => {
            render_settings(f, app, chunks[1]);
        }
    }

    render_status_bar(f, app, chunks[3]);
}

/// Render the header with session info
fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let session_name = app
        .current_session
        .as_ref()
        .and_then(|s| s.title.as_deref())
        .unwrap_or("No Session");

    let model = "claude-3-5-sonnet"; // TODO: Get from app state
    let tokens = app.total_tokens();
    let cost = app.total_cost();

    let header_text = format!(
        " Session: {} │ Model: {} │ Tokens: {} │ Cost: ${:.4}",
        session_name, model, tokens, cost
    );

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Crustly AI Assistant "));

    f.render_widget(header, area);
}

/// Render the chat messages
fn render_chat(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for msg in &app.messages {
        // Add timestamp and role
        let timestamp = msg.timestamp.format("%H:%M:%S");
        let role_style = if msg.role == "user" {
            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", timestamp), Style::default().fg(Color::DarkGray)),
            Span::styled(&msg.role, role_style),
        ]));

        // Add message content (word-wrapped)
        for line in msg.content.lines() {
            lines.push(Line::from(line.to_string()));
        }

        // Add spacing
        lines.push(Line::from(""));
    }

    // Add streaming response if present
    if let Some(ref response) = app.streaming_response {
        lines.push(Line::from(vec![
            Span::styled("[streaming] ", Style::default().fg(Color::DarkGray)),
            Span::styled("assistant", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));

        for line in response.lines() {
            lines.push(Line::from(line.to_string()));
        }
    }

    // Show processing indicator
    if app.is_processing && app.streaming_response.is_none() {
        lines.push(Line::from(vec![
            Span::styled("● Processing", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("...", Style::default().fg(Color::Yellow)),
        ]));
    }

    let chat = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Chat "))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset as u16, 0));

    f.render_widget(chat, area);
}

/// Render the input box
fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let input_lines: Vec<Line> = app
        .input_buffer
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let input = Paragraph::new(input_lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Input (Ctrl+Enter to send, Esc to clear) ")
                .border_style(if app.is_processing {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Cyan)
                }),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(input, area);
}

/// Render the sessions list
fn render_sessions(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Sessions (↑/↓ to navigate, Enter to select, Esc to cancel)",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (idx, session) in app.sessions.iter().enumerate() {
        let is_selected = idx == app.selected_session_index;
        let is_current = app
            .current_session
            .as_ref()
            .map(|s| s.id == session.id)
            .unwrap_or(false);

        let prefix = if is_selected { "> " } else { "  " };
        let suffix = if is_current { " [current]" } else { "" };

        let name = session.title.as_deref().unwrap_or("Untitled");
        let created = session.created_at.format("%Y-%m-%d %H:%M");

        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if is_current {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(Span::styled(
            format!("{}{} - {}{}", prefix, name, created, suffix),
            style,
        )));
    }

    let sessions = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Sessions "))
        .wrap(Wrap { trim: false });

    f.render_widget(sessions, area);
}

/// Render the help screen
fn render_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = vec![
        Line::from(Span::styled("Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Global:"),
        Line::from("  Ctrl+C       - Quit application"),
        Line::from("  Ctrl+N       - New session"),
        Line::from("  Ctrl+L       - List sessions"),
        Line::from("  Ctrl+H       - Show this help"),
        Line::from(""),
        Line::from("Chat Mode:"),
        Line::from("  Ctrl+Enter   - Send message"),
        Line::from("  Escape       - Clear input"),
        Line::from("  Page Up/Down - Scroll chat"),
        Line::from(""),
        Line::from("Session List:"),
        Line::from("  ↑/↓          - Navigate"),
        Line::from("  Enter        - Select session"),
        Line::from("  Escape       - Return to chat"),
        Line::from(""),
        Line::from(Span::styled("Press Esc to return", Style::default().fg(Color::Yellow))),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .alignment(Alignment::Left);

    f.render_widget(help, area);
}

/// Render the settings screen
fn render_settings(f: &mut Frame, _app: &App, area: Rect) {
    let settings_text = vec![
        Line::from(Span::styled("Settings", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Coming soon..."),
        Line::from(""),
        Line::from(Span::styled("Press Esc to return", Style::default().fg(Color::Yellow))),
    ];

    let settings = Paragraph::new(settings_text)
        .block(Block::default().borders(Borders::ALL).title(" Settings "))
        .alignment(Alignment::Left);

    f.render_widget(settings, area);
}

/// Render the status bar
fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.mode {
        AppMode::Chat => "CHAT",
        AppMode::Sessions => "SESSIONS",
        AppMode::Help => "HELP",
        AppMode::Settings => "SETTINGS",
    };

    let status = if let Some(ref error) = app.error_message {
        format!(" [{}] ERROR: {}", mode_text, error)
    } else if app.is_processing {
        format!(" [{}] Processing...", mode_text)
    } else {
        format!(" [{}] Ready │ Ctrl+H: Help │ Ctrl+L: Sessions │ Ctrl+N: New │ Ctrl+C: Quit", mode_text)
    };

    let status_color = if app.error_message.is_some() {
        Color::Red
    } else if app.is_processing {
        Color::Yellow
    } else {
        Color::Green
    };

    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(Color::Black).bg(status_color));

    f.render_widget(status_bar, area);
}
