//! TUI Rendering
//!
//! Main rendering logic for the terminal interface.

use super::app::App;
use super::events::AppMode;
use super::markdown::parse_markdown;
use super::splash;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the entire UI
pub fn render(f: &mut Frame, app: &App) {
    // Show splash screen if in splash mode
    if app.mode == AppMode::Splash {
        splash::render_splash(f, f.size(), app.provider_name(), app.provider_model());
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header (now has 2 lines)
            Constraint::Min(10),   // Main content
            Constraint::Length(5), // Input
            Constraint::Length(1), // Status bar
        ])
        .split(f.size());

    // Render components based on mode
    render_header(f, app, chunks[0]);

    match app.mode {
        AppMode::Splash => {
            // Already handled above
        }
        AppMode::Chat => {
            render_chat(f, app, chunks[1]);
            render_input(f, app, chunks[2]);
        }
        AppMode::Plan => {
            render_plan(f, app, chunks[1]);
            // Clear the input area (render help text instead)
            render_plan_help(f, chunks[2]);
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
        AppMode::ToolApproval => {
            render_approval(f, app, chunks[1]);
        }
        AppMode::FilePicker => {
            render_file_picker(f, app, chunks[1]);
        }
        AppMode::ModelDownload => {
            render_model_download(f, app, chunks[1]);
        }
    }

    render_status_bar(f, app, chunks[3]);
}

/// Purely cosmetic icon shown next to the provider name in the header.
/// Unknown provider names fall back to a generic robot.
fn provider_icon(provider_name: &str) -> &'static str {
    match provider_name {
        "ollama" => "🦙",
        "openai" => "🏠",
        "anthropic" => "🤖",
        "qwen" => "🌀",
        "azure" => "☁️",
        _ => "🤖",
    }
}

/// Render the header with session info
fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let session_name = app
        .current_session
        .as_ref()
        .and_then(|s| s.title.as_deref())
        .unwrap_or("No Session");

    let model = app
        .current_session
        .as_ref()
        .and_then(|s| s.model.as_deref())
        .unwrap_or("unknown");
    let provider = app
        .current_session
        .as_ref()
        .and_then(|s| s.provider.as_deref());
    let tokens = app.total_tokens();
    let cost = app.total_cost();
    // Throughput of the most recent assistant reply, if the active provider
    // exposes it (currently only the native Ollama provider). Omitted
    // entirely (not shown as "0 tok/s") when unavailable.
    let tokens_per_second = app.messages.iter().rev().find_map(|m| m.tokens_per_second);

    // Format working directory - show relative or full path
    let working_dir = app.working_directory.to_string_lossy().to_string();
    let display_dir = if working_dir.len() > 60 {
        format!("...{}", &working_dir[working_dir.len() - 57..])
    } else {
        working_dir
    };

    let mut header_spans = vec![
        Span::styled(" 📝 Session: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            session_name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🤖 Model: ", Style::default().fg(Color::DarkGray)),
    ];
    if let Some(provider) = provider {
        header_spans.push(Span::styled(
            format!("{} ", provider_icon(provider)),
            Style::default().fg(Color::Green),
        ));
    }
    header_spans.push(Span::styled(model, Style::default().fg(Color::Green)));
    header_spans.extend([
        Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
        Span::styled("💬 Tokens: ", Style::default().fg(Color::DarkGray)),
        Span::styled(tokens.to_string(), Style::default().fg(Color::Yellow)),
        Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
        Span::styled("💰 Cost: $", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{:.4}", cost), Style::default().fg(Color::Magenta)),
    ]);
    if let Some(tps) = tokens_per_second {
        header_spans.extend([
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("⚡ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.0} tok/s", tps),
                Style::default().fg(Color::Cyan),
            ),
        ]);
    }

    let header_line1 = Line::from(header_spans);

    let header_line2 = Line::from(vec![
        Span::styled(
            " 📁 Working Directory: ",
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            display_dir,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let header = Paragraph::new(vec![header_line1, header_line2]).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                " 🦀 Crustly AI Assistant ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(header, area);
}

/// Render the chat messages
fn render_chat(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // Show banner if there's a pending plan
    if let Some(ref plan) = app.current_plan {
        if matches!(plan.status, crate::tui::plan::PlanStatus::PendingApproval) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("  ⚠️  ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    "Plan Pending Approval",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Ctrl+P",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " to review the plan, or switch to Plan Mode to approve/reject.",
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
            lines.push(Line::from(Span::styled(
                "  ─".repeat(30),
                Style::default().fg(Color::Yellow),
            )));
            lines.push(Line::from(""));
        }
    }

    // Get the model name from the current session
    let model_name = app
        .current_session
        .as_ref()
        .and_then(|s| s.model.as_deref())
        .unwrap_or("AI");

    for msg in &app.messages {
        // Add timestamp and role with better formatting
        let timestamp = msg.timestamp.format("%H:%M:%S");

        // Build role text and style
        let (role_text, role_style, prefix) = if msg.role == "user" {
            (
                "You".to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                "  ",
            )
        } else {
            (
                format!("🤖 {}", model_name),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
                "",
            )
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, Style::default()),
            Span::styled(role_text, role_style),
            Span::styled(
                format!(" ({})", timestamp),
                Style::default().fg(Color::DarkGray),
            ),
        ]));

        // Render thinking block (collapsed by default, expanded with 't' key)
        if let Some(ref thinking) = msg.thinking_text {
            if msg.thinking_expanded {
                lines.push(Line::from(vec![
                    Span::styled(
                        "[Thinking ▾] ",
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "(press t to collapse)",
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
                for thinking_line in thinking.lines() {
                    lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::Magenta)),
                        Span::styled(thinking_line.to_string(), Style::default().fg(Color::Gray)),
                    ]));
                }
            } else {
                lines.push(Line::from(vec![
                    Span::styled(
                        "[Thinking ▸] ",
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("(press t to expand)", Style::default().fg(Color::DarkGray)),
                ]));
            }
        }

        // Parse and render message content as markdown
        let mut content_lines = parse_markdown(&msg.content);
        lines.append(&mut content_lines);

        // Runtime performance metrics footer (currently Ollama only) - shown
        // only when the provider actually reported them.
        if let Some(ref perf) = msg.perf_metrics {
            let mut spans = vec![Span::styled("  ⏱ ", Style::default().fg(Color::DarkGray))];
            if let Some(eval_ms) = perf.eval_duration_ms {
                spans.push(Span::styled(
                    format!("{}ms generation", eval_ms),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            if let Some(tps) = msg.tokens_per_second {
                spans.push(Span::styled(
                    format!(" · {:.0} tok/s", tps),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            match perf.model_was_loaded {
                Some(false) => {
                    let load_ms = perf.load_duration_ms.unwrap_or(0);
                    spans.push(Span::styled(
                        format!(" · 🧊 cold start (model loaded in {}ms)", load_ms),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
                Some(true) => {
                    spans.push(Span::styled(
                        " · 🔥 warm",
                        Style::default().fg(Color::DarkGray),
                    ));
                }
                None => {}
            }
            lines.push(Line::from(spans));
        }

        // Add spacing between messages
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─".repeat(60),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
    }

    // Add streaming response if present
    if let Some(ref response) = app.streaming_response {
        lines.push(Line::from(vec![
            Span::styled(
                format!("🤖 {} ", model_name),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("[streaming]", Style::default().fg(Color::DarkGray)),
        ]));

        let mut streaming_lines = parse_markdown(response);
        lines.append(&mut streaming_lines);
    }

    // Show processing indicator with animated spinner
    if app.is_processing && app.streaming_response.is_none() {
        let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame = spinner_frames[app.animation_frame % spinner_frames.len()];

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", frame),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} is thinking...", model_name),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    // Calculate scroll offset for ratatui
    // app.scroll_offset represents "lines scrolled up from the bottom"
    // 0 = at the bottom (auto-scroll, showing latest messages)
    // N = scrolled up N lines from the bottom (showing older messages)
    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // Subtract borders
    let max_scroll = total_lines.saturating_sub(visible_height);
    let actual_scroll_offset = max_scroll.saturating_sub(app.scroll_offset);

    let chat = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " 💬 Chat ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
        .scroll((actual_scroll_offset as u16, 0));

    f.render_widget(chat, area);
}

/// Render the input box
fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let mut input_text = app.input_buffer.clone();

    // Add cursor indicator
    if !app.is_processing {
        input_text.push('█');
    }

    let input_lines: Vec<Line> = input_text
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let title = if app.is_processing {
        Span::styled(
            " ⏸️  Input (waiting for response...) ",
            Style::default().fg(Color::DarkGray),
        )
    } else {
        Span::styled(
            " ✏️  Type your message (Ctrl+Enter to send, Esc to clear) ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    };

    let border_style = if app.is_processing {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let input = Paragraph::new(input_lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(input, area);
}

/// Render the sessions list
fn render_sessions(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Sessions (↑/↓ to navigate, Enter to select, Esc to cancel)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
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
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
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
        Line::from(vec![
            Span::styled("🥐 ", Style::default().fg(Color::Rgb(218, 165, 32))),
            Span::styled(
                "Crustly Help",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "╭─ GLOBAL COMMANDS ─────────────────────────────────────────╮",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Ctrl+C       ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Quit application", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+N       ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Create new chat session", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+L       ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "List all sessions (switch sessions)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+H       ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Show this help screen", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+K       ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Clear current session messages",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "╭─ CHAT MODE ───────────────────────────────────────────────╮",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Ctrl+Enter   ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Send message to LLM", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter        ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "New line in message (multi-line input)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Backspace    ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Delete last character", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Escape       ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Clear input buffer", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Page Up      ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Scroll chat history up", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Page Down    ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Scroll chat history down",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "╭─ SESSION LIST ────────────────────────────────────────────╮",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ↑/↓          ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Navigate through sessions",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter        ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Load selected session", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Escape       ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Return to chat", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "╭─ PLAN MODE ───────────────────────────────────────────────╮",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Ctrl+P       ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled("Toggle Plan Mode view", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+A       ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Approve plan and start execution",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+R       ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Reject plan and return to chat",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↑/↓          ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Scroll through plan tasks",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Page Up/Down ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Scroll plan tasks faster",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "╭─ FEATURES ────────────────────────────────────────────────╮",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Markdown Rendering",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - Rich text with headings, lists, code",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Syntax Highlighting",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - 100+ languages supported",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Multi-line Input",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - Write long messages with ease",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Session Management",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - Persistent conversation history",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Streaming Responses",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - See responses as they're generated",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Token & Cost Tracking",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - Monitor usage in real-time",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ✓ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Plan Mode",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " - Structured task planning with approval",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "                    Press ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to return to chat", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " 📚 Help & Commands ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(help, area);
}

/// Render help text in the input area during Plan Mode
fn render_plan_help(f: &mut Frame, area: Rect) {
    let help_text = vec![Line::from(vec![
        Span::styled(
            "[Ctrl+A] ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Approve & Execute  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Ctrl+R] ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Reject  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Ctrl+I] ",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Request Changes  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Esc] ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Back  ", Style::default().fg(Color::White)),
        Span::styled(
            "[↑↓] ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Scroll", Style::default().fg(Color::White)),
    ])];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Plan Mode - Review & Approve ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

/// Render the plan mode view
#[allow(clippy::vec_init_then_push)]
fn render_plan(f: &mut Frame, app: &App, area: Rect) {
    if let Some(plan) = &app.current_plan {
        // Render the plan document
        let mut lines = vec![];

        // Plan header
        lines.push(Line::from(vec![
            Span::styled("📋 ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &plan.title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // Status
        lines.push(Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(plan.status.to_string(), Style::default().fg(Color::Yellow)),
        ]));

        lines.push(Line::from(""));

        // Description
        if !plan.description.is_empty() {
            lines.push(Line::from(Span::styled(
                "📝 Description:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                &plan.description,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        // Technical Stack
        if !plan.technical_stack.is_empty() {
            lines.push(Line::from(Span::styled(
                "🛠️  Technical Stack:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )));
            for tech in &plan.technical_stack {
                lines.push(Line::from(vec![
                    Span::styled("    • ", Style::default().fg(Color::DarkGray)),
                    Span::styled(tech, Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Test Strategy
        if !plan.test_strategy.is_empty() {
            lines.push(Line::from(Span::styled(
                "🧪 Test Strategy:",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                &plan.test_strategy,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        // Tasks
        lines.push(Line::from(Span::styled(
            format!("📋 Tasks ({}):", plan.tasks.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        for (idx, task) in plan.tasks.iter().enumerate() {
            // Task line
            lines.push(Line::from(vec![
                Span::styled(format!(" {} ", task.status.icon()), Style::default()),
                Span::styled(
                    format!("{}. ", idx + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&task.title, Style::default().fg(Color::White)),
            ]));

            // Task details (type and complexity)
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
                Span::styled(task.task_type.to_string(), Style::default().fg(Color::Cyan)),
                Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
                Span::styled("Complexity: ", Style::default().fg(Color::DarkGray)),
                Span::styled(task.complexity_stars(), Style::default().fg(Color::Yellow)),
            ]));

            // Acceptance Criteria
            if !task.acceptance_criteria.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("    ", Style::default()),
                    Span::styled("✓ Acceptance Criteria:", Style::default().fg(Color::Green)),
                ]));
                for criterion in &task.acceptance_criteria {
                    lines.push(Line::from(vec![
                        Span::styled("      • ", Style::default().fg(Color::DarkGray)),
                        Span::styled(criterion, Style::default().fg(Color::White)),
                    ]));
                }
            }

            lines.push(Line::from(""));
        }

        // Action bar
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(vec![
            Span::styled("[Ctrl+A] ", Style::default().fg(Color::Green)),
            Span::styled("Approve  ", Style::default().fg(Color::White)),
            Span::styled("[Ctrl+R] ", Style::default().fg(Color::Yellow)),
            Span::styled("Reject  ", Style::default().fg(Color::White)),
            Span::styled("[Esc] ", Style::default().fg(Color::Red)),
            Span::styled("Cancel", Style::default().fg(Color::White)),
        ]));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 📋 PLAN MODE ")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: false })
            .scroll((app.plan_scroll_offset as u16, 0));

        f.render_widget(paragraph, area);
    } else {
        // No plan available
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "📋 Plan Mode",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "No active plan. Switch to Chat mode to create a plan.",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }
}

/// Render the settings screen
fn render_settings(f: &mut Frame, _app: &App, area: Rect) {
    let settings_text = vec![
        Line::from(Span::styled(
            "Settings",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Coming soon..."),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to return",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let settings = Paragraph::new(settings_text)
        .block(Block::default().borders(Borders::ALL).title(" Settings "))
        .alignment(Alignment::Left);

    f.render_widget(settings, area);
}

/// Render the tool approval dialog
fn render_approval(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref request) = app.pending_approval {
        // Get the model name from the current session
        let model_name = app
            .current_session
            .as_ref()
            .and_then(|s| s.model.as_deref())
            .unwrap_or("AI");
        // Center the dialog
        let dialog_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(if app.show_approval_details { 30 } else { 20 }),
                Constraint::Min(0),
            ])
            .split(area);

        let center_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(80),
                Constraint::Min(0),
            ])
            .split(dialog_chunks[1]);

        let dialog_area = center_chunks[1];

        // Build dialog content - calculate time remaining
        let time_remaining = request.time_remaining();
        let seconds_remaining = time_remaining.as_secs();
        let time_color = if seconds_remaining < 60 {
            Color::Red
        } else if seconds_remaining < 180 {
            Color::Yellow
        } else {
            Color::Green
        };

        let mut lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("🔒 ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    "Permission Request",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
                Span::styled("⏱️  ", Style::default().fg(time_color)),
                Span::styled(
                    format!(
                        "{}m {}s remaining",
                        seconds_remaining / 60,
                        seconds_remaining % 60
                    ),
                    Style::default().fg(time_color),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{} wants to use the tool: ", model_name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    &request.tool_name,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&request.tool_description, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
        ];

        // Show capabilities
        if !request.capabilities.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "⚠️  Capabilities: ",
                Style::default().fg(Color::Yellow),
            )]));
            for cap in &request.capabilities {
                lines.push(Line::from(vec![
                    Span::styled("   • ", Style::default().fg(Color::DarkGray)),
                    Span::styled(cap, Style::default().fg(Color::Red)),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Show input parameters (basic or detailed)
        if app.show_approval_details {
            lines.push(Line::from(vec![Span::styled(
                "Tool Input (JSON):",
                Style::default().fg(Color::DarkGray),
            )]));
            lines.push(Line::from(""));
            let json_str = serde_json::to_string_pretty(&request.tool_input)
                .unwrap_or_else(|_| "{}".to_string());
            for line in json_str.lines() {
                lines.push(Line::from(vec![Span::styled(
                    line.to_string(),
                    Style::default().fg(Color::Green),
                )]));
            }
            lines.push(Line::from(""));
        } else {
            // Show simplified input
            if let Some(obj) = request.tool_input.as_object() {
                if !obj.is_empty() {
                    lines.push(Line::from(vec![Span::styled(
                        "Parameters: ",
                        Style::default().fg(Color::DarkGray),
                    )]));
                    for (key, value) in obj.iter().take(3) {
                        let value_str = match value {
                            serde_json::Value::String(s) => {
                                if s.len() > 50 {
                                    format!("\"{}...\"", &s[..47])
                                } else {
                                    format!("\"{}\"", s)
                                }
                            }
                            _ => value.to_string(),
                        };
                        lines.push(Line::from(vec![
                            Span::styled(format!("   {}: ", key), Style::default().fg(Color::Cyan)),
                            Span::styled(value_str, Style::default().fg(Color::White)),
                        ]));
                    }
                    if obj.len() > 3 {
                        lines.push(Line::from(vec![
                            Span::styled("   ... ", Style::default().fg(Color::DarkGray)),
                            Span::styled(
                                format!("({} more)", obj.len() - 3),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]));
                    }
                    lines.push(Line::from(""));
                }
            }
        }

        // Show action buttons
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                "[A]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("pprove  ", Style::default().fg(Color::White)),
            Span::styled(
                "[D]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("eny  ", Style::default().fg(Color::White)),
            Span::styled(
                "[V]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("iew Details  ", Style::default().fg(Color::White)),
            Span::styled(
                "[Esc]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Cancel", Style::default().fg(Color::White)),
        ]));

        let dialog = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red))
                    .title(Span::styled(
                        " ⚠️  PERMISSION REQUIRED ",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )),
            )
            .alignment(Alignment::Left);

        f.render_widget(dialog, dialog_area);
    }
}

/// Render the file picker
fn render_file_picker(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // Header
    lines.push(Line::from(vec![
        Span::styled(
            "📁 File Picker",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.file_picker_current_dir.to_string_lossy().to_string(),
            Style::default().fg(Color::Yellow),
        ),
    ]));
    lines.push(Line::from(""));

    // Calculate visible range
    let visible_items = (area.height as usize).saturating_sub(6); // Leave space for header and help
    let start = app.file_picker_scroll_offset;
    let end = (start + visible_items).min(app.file_picker_files.len());

    // Render file list
    for (idx, path) in app
        .file_picker_files
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
    {
        let is_selected = idx == app.file_picker_selected;
        let is_dir = path.is_dir();

        let icon = if path.ends_with("..") {
            "📂 .."
        } else if is_dir {
            "📂"
        } else {
            "📄"
        };

        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");

        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if is_dir {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if is_selected { "▶ " } else { "  " };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{} {}", icon, filename), style),
        ]));
    }

    // Add scroll indicator if needed
    if app.file_picker_files.len() > visible_items {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            format!(
                "Showing {}-{} of {} files",
                start + 1,
                end,
                app.file_picker_files.len()
            ),
            Style::default().fg(Color::DarkGray),
        )]));
    }

    // Help text
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            "[↑↓]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Navigate  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Enter]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Select  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Esc]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Cancel", Style::default().fg(Color::White)),
    ]));

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Select a file ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the Model Download dialog (Ctrl+D): either the model
/// name input + suggestions list, or a live progress bar while a pull is
/// in flight.
fn render_model_download(f: &mut Frame, app: &App, area: Rect) {
    if app.model_download_running {
        render_model_download_progress(f, app, area);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![Span::styled(
        "🦙 Download an Ollama model",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  > ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if app.model_download_input.is_empty() {
                "type a model name, e.g. qwen2.5-coder:7b"
            } else {
                app.model_download_input.as_str()
            },
            if app.model_download_input.is_empty() {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            },
        ),
    ]));
    lines.push(Line::from(""));

    if app.model_download_suggestions.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No matches - press Enter to pull this name anyway.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, name) in app.model_download_suggestions.iter().enumerate() {
            let is_selected = idx == app.model_download_selected;
            let is_installed = app.model_download_installed.iter().any(|m| m == name);
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if is_selected { "▶ " } else { "  " };
            let status = if is_installed { " (installed)" } else { "" };
            lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{}{}", name, status), style),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            "[↑↓]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Navigate  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Tab]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Use suggestion  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Enter]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Pull  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Esc]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Cancel", Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(Span::styled(
        "Note: Ollama has no online search API - type any repo:tag you know, or pick a suggestion.",
        Style::default().fg(Color::DarkGray),
    )));

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Download Model ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the live progress view of an in-flight model pull.
fn render_model_download_progress(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![Span::styled(
        format!("🦙 Downloading '{}'", app.model_download_input),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));

    let status = app.model_download_status.as_deref().unwrap_or("working…");
    lines.push(Line::from(vec![Span::styled(
        format!("  {}", status),
        Style::default().fg(Color::White),
    )]));

    if let Some(fraction) = app.model_download_fraction {
        const BAR_WIDTH: usize = 30;
        let filled = ((fraction * BAR_WIDTH as f64).round() as usize).min(BAR_WIDTH);
        let bar = format!(
            "[{}{}] {:>3.0}%",
            "█".repeat(filled),
            "░".repeat(BAR_WIDTH - filled),
            fraction * 100.0
        );
        lines.push(Line::from(vec![Span::styled(
            format!("  {}", bar),
            Style::default().fg(Color::Green),
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  (Esc cancels the download)",
        Style::default().fg(Color::DarkGray),
    )));

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(
                    " Downloading… ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the status bar
fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.mode {
        AppMode::Splash => "WELCOME",
        AppMode::Chat => "CHAT",
        AppMode::Plan => "PLAN",
        AppMode::Sessions => "SESSIONS",
        AppMode::Help => "HELP",
        AppMode::Settings => "SETTINGS",
        AppMode::ToolApproval => "PERMISSION",
        AppMode::FilePicker => "FILE PICKER",
        AppMode::ModelDownload => "MODEL DOWNLOAD",
    };

    let status = if let Some(ref error) = app.error_message {
        format!(" [{}] ERROR: {}", mode_text, error)
    } else if app.is_processing {
        format!(" [{}] Processing...", mode_text)
    } else {
        format!(
            " [{}] Ready │ Ctrl+H: Help │ Ctrl+D: Download Model │ Ctrl+K: Clear │ Ctrl+L: Sessions │ Ctrl+N: New │ Ctrl+C: Quit",
            mode_text
        )
    };

    let status_color = if app.error_message.is_some() {
        Color::Red
    } else if app.is_processing {
        Color::Yellow
    } else {
        Color::Green
    };

    let status_bar =
        Paragraph::new(status).style(Style::default().fg(Color::Black).bg(status_color));

    f.render_widget(status_bar, area);
}
