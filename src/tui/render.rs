//! TUI Rendering
//!
//! Main rendering logic for the terminal interface.

use super::app::App;
use super::events::AppMode;
use super::markdown::{parse_markdown, parse_plain_text};
use super::splash;
use crate::config::PlanExecMode;
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
        splash::render_splash(f, f.area(), app.provider_name(), app.provider_model());
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
        .split(f.area());

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
        AppMode::ModelInfo => {
            render_model_info(f, app, chunks[1]);
        }
        AppMode::ProviderSwitch => {
            render_provider_switch(f, app, chunks[1]);
        }
        AppMode::LlamaCppModelPicker => {
            render_llama_cpp_models(f, app, chunks[1]);
        }
        AppMode::Skills => {
            render_skills(f, app, chunks[1]);
        }
        AppMode::Mcp => {
            render_mcp(f, app, chunks[1]);
        }
    }

    render_status_bar(f, app, chunks[3]);
}

/// Purely cosmetic icon shown next to the provider name in the header.
/// Unknown provider names fall back to a generic robot.
fn provider_icon(provider_name: &str) -> &'static str {
    match provider_name {
        "ollama" => "🦙",
        "llama-cpp" => "⚙️",
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

/// Banner shown above the chat log while a plan is awaiting approval.
fn render_pending_plan_banner(app: &App) -> Vec<Line<'static>> {
    let Some(ref plan) = app.current_plan else {
        return Vec::new();
    };
    if !matches!(plan.status, crate::plan::PlanStatus::PendingApproval) {
        return Vec::new();
    }

    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ⚠️  ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Plan Pending Approval",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
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
        ]),
        Line::from(Span::styled(
            "  ─".repeat(30),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
    ]
}

/// The runtime performance footer (currently Ollama only) shown under a
/// message, when the provider actually reported timing metrics.
fn render_perf_footer(msg: &super::app::DisplayMessage) -> Option<Line<'static>> {
    let perf = msg.perf_metrics.as_ref()?;

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
    Some(Line::from(spans))
}

/// Render the collapsed/expanded "thinking" block for a message, if present.
fn render_thinking_block(msg: &super::app::DisplayMessage) -> Vec<Line<'static>> {
    let Some(ref thinking) = msg.thinking_text else {
        return Vec::new();
    };

    if !msg.thinking_expanded {
        return vec![Line::from(vec![
            Span::styled(
                "[Thinking ▸] ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("(press t to expand)", Style::default().fg(Color::DarkGray)),
        ])];
    }

    let mut lines = vec![Line::from(vec![
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
    ])];
    for thinking_line in thinking.lines() {
        lines.push(Line::from(vec![
            Span::styled("  │ ", Style::default().fg(Color::Magenta)),
            Span::styled(thinking_line.to_string(), Style::default().fg(Color::Gray)),
        ]));
    }
    lines
}

/// Render one full message: role/timestamp header, thinking block, body, perf
/// footer, and the trailing separator.
fn render_message_lines(msg: &super::app::DisplayMessage, model_name: &str) -> Vec<Line<'static>> {
    // Timestamps are stored in UTC; show them in the user's local timezone so
    // the clock in the transcript matches the wall clock on their machine.
    let timestamp = msg
        .timestamp
        .with_timezone(&chrono::Local)
        .format("%H:%M:%S");
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

    let mut lines = vec![Line::from(vec![
        Span::styled(prefix, Style::default()),
        Span::styled(role_text, role_style),
        Span::styled(
            format!(" ({})", timestamp),
            Style::default().fg(Color::DarkGray),
        ),
    ])];

    lines.extend(render_thinking_block(msg));

    // Assistant replies are markdown and are rendered as such. The user's own
    // message is not - it is literal text, and parsing it as CommonMark eats
    // the backslashes out of any Windows path they typed or pasted.
    if msg.role == "user" {
        lines.extend(parse_plain_text(&msg.content));
    } else {
        lines.extend(parse_markdown(&msg.content));
    };

    lines.extend(render_perf_footer(msg));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "─".repeat(60),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    lines
}

/// The in-progress assistant reply, streamed token-by-token.
fn render_streaming_response(app: &App, model_name: &str) -> Vec<Line<'static>> {
    let Some(ref response) = app.streaming_response else {
        return Vec::new();
    };

    let mut lines = vec![Line::from(vec![
        Span::styled(
            format!("🤖 {} ", model_name),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("[streaming]", Style::default().fg(Color::DarkGray)),
    ])];
    lines.extend(parse_markdown(response));
    lines
}

/// Animated "model is thinking" spinner, shown while waiting for the first
/// streamed token.
fn render_processing_indicator(app: &App, model_name: &str) -> Vec<Line<'static>> {
    if !app.is_processing || app.streaming_response.is_some() {
        return Vec::new();
    }

    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frame = spinner_frames[app.animation_frame % spinner_frames.len()];

    vec![
        Line::from(""),
        Line::from(vec![
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
        ]),
    ]
}

/// `app.scroll_offset` is "lines scrolled up from the bottom" (0 = auto-scroll
/// showing the latest messages); ratatui's `Paragraph::scroll` wants an
/// absolute offset from the top, so convert between the two here.
fn compute_scroll_offset(total_lines: usize, visible_height: usize, scroll_offset: usize) -> u16 {
    let max_scroll = total_lines.saturating_sub(visible_height);
    max_scroll.saturating_sub(scroll_offset) as u16
}

/// Render the chat messages
fn render_chat(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.extend(render_pending_plan_banner(app));

    // Get the model name from the current session
    let model_name = app
        .current_session
        .as_ref()
        .and_then(|s| s.model.as_deref())
        .unwrap_or("AI");

    for msg in &app.messages {
        lines.extend(render_message_lines(msg, model_name));
    }

    lines.extend(render_streaming_response(app, model_name));
    lines.extend(render_processing_indicator(app, model_name));

    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // Subtract borders
    let actual_scroll_offset =
        compute_scroll_offset(total_lines, visible_height, app.scroll_offset);

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
        .scroll((actual_scroll_offset, 0));

    f.render_widget(chat, area);
}

/// Render the input box. Clones `app.textarea` rather than mutating it in
/// place, since every other `render_*` function takes `app: &App` and this
/// keeps that read-only convention intact (`TextArea::set_block`/
/// `set_cursor_style` need `&mut self`, but only to apply per-frame
/// styling, not to change the actual buffer contents).
fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let title = if app.is_processing {
        Span::styled(
            " ⏸️  Input (waiting for response...) ",
            Style::default().fg(Color::DarkGray),
        )
    } else {
        let newline_hint = if app.kitty_keyboard_protocol_active {
            "Shift+Enter for newline"
        } else {
            "Alt+Enter for newline"
        };
        Span::styled(
            format!(
                " ✏️  Type your message (Enter to send, {newline_hint}, ↑ history, Esc to clear) "
            ),
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

    let mut textarea = app.textarea.clone();
    textarea.set_style(Style::default().fg(Color::White));
    if app.is_processing {
        // No visible cursor while a response is in flight, matching the
        // previous behavior of not appending a cursor glyph.
        textarea.set_cursor_style(Style::default());
    }
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style),
    );

    f.render_widget(&textarea, area);
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
        // UTC in the DB -> local for display (see render_message_lines).
        let created = session
            .created_at
            .with_timezone(&chrono::Local)
            .format("%Y-%m-%d %H:%M");

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

/// Render the `/skills` list view.
fn render_skills(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Discoverable skills (↑/↓ to navigate, Esc to close)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    if app.skills_list.is_empty() {
        lines.push(Line::from(Span::styled(
            "No skills found in .crustly/skills, .claude/skills, or their user-global equivalents.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, skill) in app.skills_list.iter().enumerate() {
            let is_selected = idx == app.skills_selected;
            let prefix = if is_selected { "> " } else { "  " };
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(Span::styled(
                format!("{prefix}{}", skill.name),
                style,
            )));
            if let Some(desc) = &skill.description {
                lines.push(Line::from(Span::styled(
                    format!("      {desc}"),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
    }

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " /skills ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the `/mcp` list view.
fn render_mcp(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Configured MCP servers (↑/↓ to navigate, Esc to close)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    if app.mcp_status.is_empty() {
        lines.push(Line::from(Span::styled(
            "No MCP servers configured. Add entries under [[mcp.servers]] in config.toml.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, server) in app.mcp_status.iter().enumerate() {
            let is_selected = idx == app.mcp_selected;
            let prefix = if is_selected { "> " } else { "  " };
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let status = if server.connected {
                format!("connected, {} tools", server.tool_count)
            } else {
                "not connected".to_string()
            };

            lines.push(Line::from(Span::styled(
                format!("{prefix}{} — {} ({status})", server.name, server.command),
                style,
            )));
            if let Some(err) = &server.error {
                lines.push(Line::from(Span::styled(
                    format!("      {err}"),
                    Style::default().fg(Color::Red),
                )));
            }
        }
    }

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " /mcp ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// A single "key → description" row, the recurring building block of the help screen.
fn help_row(key: &'static str, desc: impl Into<String>, key_color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            key,
            Style::default().fg(key_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("→ ", Style::default().fg(Color::DarkGray)),
        Span::styled(desc.into(), Style::default().fg(Color::White)),
    ])
}

/// A single "✓ Name - description" row used in the FEATURES section.
fn feature_row(name: &'static str, desc: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled("  ✓ ", Style::default().fg(Color::Green)),
        Span::styled(
            name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(desc, Style::default().fg(Color::DarkGray)),
    ])
}

/// Blank line + boxed section title + blank line, shared by every help section.
fn help_section_header(title: &'static str) -> [Line<'static>; 3] {
    [
        Line::from(""),
        Line::from(Span::styled(title, Style::default().fg(Color::Cyan))),
        Line::from(""),
    ]
}

fn help_global_commands() -> Vec<Line<'static>> {
    let mut lines =
        help_section_header("╭─ GLOBAL COMMANDS ─────────────────────────────────────────╮")
            .to_vec();
    lines.extend([
        help_row("  Ctrl+C       ", "Quit application", Color::Yellow),
        help_row("  Ctrl+N       ", "Create new chat session", Color::Yellow),
        help_row(
            "  Ctrl+L       ",
            "List all sessions (switch sessions)",
            Color::Yellow,
        ),
        help_row("  Ctrl+H       ", "Show this help screen", Color::Yellow),
        help_row(
            "  Ctrl+K       ",
            "Clear current session messages",
            Color::Yellow,
        ),
        help_row(
            "  Ctrl+O       ",
            "Show Model Info panel (provider, model, context, perf)",
            Color::Yellow,
        ),
        help_row(
            "  Ctrl+W       ",
            "Switch to a different local Ollama model",
            Color::Yellow,
        ),
        help_row(
            "  Ctrl+G       ",
            "Local llama.cpp models: switch, download, or delete",
            Color::Yellow,
        ),
        help_row(
            "  Shift+Tab    ",
            "Cycle Auto Mode: Interactive → AutoPlan → FullAuto",
            Color::Yellow,
        ),
    ]);
    lines
}

fn help_chat_mode(app: &App) -> Vec<Line<'static>> {
    let newline_key = if app.kitty_keyboard_protocol_active {
        "  Shift+Enter  "
    } else {
        "  Alt+Enter    "
    };

    let mut lines =
        help_section_header("╭─ CHAT MODE ───────────────────────────────────────────────╮")
            .to_vec();
    lines.extend([
        help_row("  Enter        ", "Send message to LLM", Color::Green),
        help_row(
            newline_key,
            "New line in message (multi-line input)",
            Color::Green,
        ),
        help_row(
            "  ↑ / ↓        ",
            "Recall previous messages (edit and resend; moves the cursor in a multi-line draft)",
            Color::Green,
        ),
        help_row(
            "  Ctrl+Enter   ",
            "Send message (legacy alias)",
            Color::Green,
        ),
        help_row(
            "  ←/→/↑/↓      ",
            "Move cursor (Ctrl+←/→ jumps by word)",
            Color::Green,
        ),
        help_row("  Home/End     ", "Jump to start/end of line", Color::Green),
        help_row(
            "  Backspace    ",
            "Delete character at cursor (Ctrl+Backspace/Delete: whole word)",
            Color::Green,
        ),
        help_row(
            "  Ctrl+Y       ",
            "Copy last response (or its code block) to clipboard",
            Color::Green,
        ),
        help_row(
            "  Ctrl+V       ",
            "Paste from system clipboard at cursor",
            Color::Green,
        ),
        help_row("  Escape       ", "Clear input buffer", Color::Green),
        help_row("  Page Up      ", "Scroll chat history up", Color::Green),
        help_row("  Page Down    ", "Scroll chat history down", Color::Green),
    ]);
    lines
}

fn help_session_list() -> Vec<Line<'static>> {
    let mut lines =
        help_section_header("╭─ SESSION LIST ────────────────────────────────────────────╮")
            .to_vec();
    lines.extend([
        help_row(
            "  ↑/↓          ",
            "Navigate through sessions",
            Color::Magenta,
        ),
        help_row("  Enter        ", "Load selected session", Color::Magenta),
        help_row("  Escape       ", "Return to chat", Color::Magenta),
    ]);
    lines
}

fn help_plan_mode() -> Vec<Line<'static>> {
    let mut lines =
        help_section_header("╭─ PLAN MODE ───────────────────────────────────────────────╮")
            .to_vec();
    lines.extend([
        help_row("  Ctrl+P       ", "Toggle Plan Mode view", Color::Blue),
        help_row(
            "  Ctrl+A       ",
            "Approve plan and start execution",
            Color::Blue,
        ),
        help_row(
            "  Ctrl+R       ",
            "Reject plan and return to chat",
            Color::Blue,
        ),
        help_row("  ↑/↓          ", "Scroll through plan tasks", Color::Blue),
        help_row("  Page Up/Down ", "Scroll plan tasks faster", Color::Blue),
    ]);
    lines
}

fn help_features() -> Vec<Line<'static>> {
    let mut lines =
        help_section_header("╭─ FEATURES ────────────────────────────────────────────────╮")
            .to_vec();
    lines.extend([
        feature_row(
            "Markdown Rendering",
            " - Rich text with headings, lists, code",
        ),
        feature_row("Syntax Highlighting", " - 100+ languages supported"),
        feature_row("Multi-line Input", " - Write long messages with ease"),
        feature_row("Session Management", " - Persistent conversation history"),
        feature_row(
            "Streaming Responses",
            " - See responses as they're generated",
        ),
        feature_row("Token & Cost Tracking", " - Monitor usage in real-time"),
        feature_row("Plan Mode", " - Structured task planning with approval"),
    ]);
    lines
}

fn help_footer() -> Vec<Line<'static>> {
    vec![
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
    ]
}

/// Render the help screen
fn render_help(f: &mut Frame, app: &App, area: Rect) {
    let mut help_text = vec![
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
    ];
    help_text.extend(help_global_commands());
    help_text.extend(help_chat_mode(app));
    help_text.extend(help_session_list());
    help_text.extend(help_plan_mode());
    help_text.extend(help_features());
    help_text.extend(help_footer());

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
/// One task entry in the plan document: title/status line, type/complexity
/// line, and its acceptance criteria (if any).
fn render_plan_task_lines(task: &crate::plan::PlanTask, idx: usize) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(vec![
            Span::styled(format!(" {} ", task.status.icon()), Style::default()),
            Span::styled(
                format!("{}. ", idx + 1),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(&task.title, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
            Span::styled(task.task_type.to_string(), Style::default().fg(Color::Cyan)),
            Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Complexity: ", Style::default().fg(Color::DarkGray)),
            Span::styled(task.complexity_stars(), Style::default().fg(Color::Yellow)),
        ]),
    ];

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
    lines
}

/// Full plan document body: header, status, description, tech stack, test
/// strategy, task list, and the bottom action bar.
fn render_plan_document(plan: &crate::plan::PlanDocument, area_width: usize) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(vec![
            Span::styled("📋 ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &plan.title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(plan.status.to_string(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
    ];

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

    lines.push(Line::from(Span::styled(
        format!("📋 Tasks ({}):", plan.tasks.len()),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (idx, task) in plan.tasks.iter().enumerate() {
        lines.extend(render_plan_task_lines(task, idx));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "─".repeat(area_width),
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

    lines
}

/// Placeholder shown in Plan Mode when there is no active plan yet.
fn render_plan_empty_state() -> Vec<Line<'static>> {
    vec![
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
    ]
}

fn render_plan(f: &mut Frame, app: &App, area: Rect) {
    if let Some(plan) = &app.current_plan {
        let lines = render_plan_document(plan, area.width as usize);

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
        let paragraph = Paragraph::new(render_plan_empty_state())
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
/// Centers the approval dialog within `area`, sized to fit the extra JSON
/// detail lines when `show_details` is on.
fn approval_dialog_area(area: Rect, show_details: bool) -> Rect {
    let dialog_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(if show_details { 30 } else { 20 }),
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

    center_chunks[1]
}

/// Header lines: time-remaining countdown, tool name, and description.
fn render_approval_header<'a>(
    request: &'a super::events::ToolApprovalRequest,
    model_name: &'a str,
) -> Vec<Line<'a>> {
    let seconds_remaining = request.time_remaining().as_secs();
    let time_color = if seconds_remaining < 60 {
        Color::Red
    } else if seconds_remaining < 180 {
        Color::Yellow
    } else {
        Color::Green
    };

    vec![
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
    ]
}

/// The "⚠️ Capabilities:" bullet list, empty when the tool declares none.
fn render_approval_capabilities(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>> {
    if request.capabilities.is_empty() {
        return Vec::new();
    }

    let mut lines = vec![Line::from(vec![Span::styled(
        "⚠️  Capabilities: ",
        Style::default().fg(Color::Yellow),
    )])];
    for cap in &request.capabilities {
        lines.push(Line::from(vec![
            Span::styled("   • ", Style::default().fg(Color::DarkGray)),
            Span::styled(cap, Style::default().fg(Color::Red)),
        ]));
    }
    lines.push(Line::from(""));
    lines
}

/// Full pretty-printed JSON of the tool input, used when the user has
/// toggled "View Details".
fn render_approval_input_detailed(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Tool Input (JSON):",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
    ];
    let json_str =
        serde_json::to_string_pretty(&request.tool_input).unwrap_or_else(|_| "{}".to_string());
    for line in json_str.lines() {
        lines.push(Line::from(vec![Span::styled(
            line.to_string(),
            Style::default().fg(Color::Green),
        )]));
    }
    lines.push(Line::from(""));
    lines
}

/// The first three tool-input parameters (truncated), used in the compact
/// (non-detailed) view of the approval dialog.
fn render_approval_input_summary(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>> {
    let Some(obj) = request.tool_input.as_object() else {
        return Vec::new();
    };
    if obj.is_empty() {
        return Vec::new();
    }

    let mut lines = vec![Line::from(vec![Span::styled(
        "Parameters: ",
        Style::default().fg(Color::DarkGray),
    )])];
    for (key, value) in obj.iter().take(3) {
        let value_str = match value {
            serde_json::Value::String(s) => {
                if s.len() > 50 {
                    format!("\"{}...\"", crate::utils::truncate_at_char_boundary(s, 47))
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
    lines
}

/// The bottom `[A]pprove [D]eny [V]iew Details [Esc] Cancel` action row.
fn render_approval_actions() -> Line<'static> {
    Line::from(vec![
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
    ])
}

fn render_approval(f: &mut Frame, app: &App, area: Rect) {
    let Some(ref request) = app.pending_approval else {
        return;
    };

    let model_name = app
        .current_session
        .as_ref()
        .and_then(|s| s.model.as_deref())
        .unwrap_or("AI");
    let dialog_area = approval_dialog_area(area, app.show_approval_details);

    let mut lines = render_approval_header(request, model_name);
    lines.extend(render_approval_capabilities(request));
    if app.show_approval_details {
        lines.extend(render_approval_input_detailed(request));
    } else {
        lines.extend(render_approval_input_summary(request));
    }
    lines.push(Line::from(""));
    lines.push(render_approval_actions());

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

/// Render the Model Info panel (Ctrl+O): active provider/model, context
/// window, and the last response's performance metrics, if any.
fn render_model_info(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![
        Span::styled("Provider: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!(
                "{} {}",
                provider_icon(app.provider_name()),
                app.provider_name()
            ),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Model:    ", Style::default().fg(Color::DarkGray)),
        Span::styled(app.provider_model(), Style::default().fg(Color::White)),
    ]));
    let context_window = app
        .provider_context_window()
        .map(|n| format!("{n} tokens"))
        .unwrap_or_else(|| "unknown".to_string());
    lines.push(Line::from(vec![
        Span::styled("Context:  ", Style::default().fg(Color::DarkGray)),
        Span::styled(context_window, Style::default().fg(Color::White)),
    ]));

    if let Some(details) = app.llama_cpp_model_details() {
        lines.push(Line::from(vec![
            Span::styled("GPU:      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if details.n_gpu_layers > 0 {
                    format!("{} layers offloaded", details.n_gpu_layers)
                } else {
                    "CPU only".to_string()
                },
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Quant:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                details
                    .quantization_hint
                    .unwrap_or_else(|| "unknown".to_string()),
                Style::default().fg(Color::White),
            ),
        ]));
        if let Some(native_ctx) = details.context_length {
            lines.push(Line::from(vec![
                Span::styled("Nat. ctx: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{native_ctx} tokens"),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::styled("Chat tmpl:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if details.has_chat_template {
                    " yes"
                } else {
                    " no"
                },
                Style::default().fg(Color::White),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Last response performance",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));

    // Look up the last assistant message once and reuse it, rather than
    // scanning app.messages twice (once for perf_metrics, once for
    // tokens_per_second) on every render.
    let last_msg = app.last_assistant_message();
    match last_msg.and_then(|m| m.perf_metrics.as_ref()) {
        Some(perf) => {
            // Matches the `{ms}ms` (no space) convention used by the
            // per-message performance footer in render_chat.
            let ms = |v: Option<u64>| {
                v.map(|ms| format!("{ms}ms"))
                    .unwrap_or_else(|| "n/a".to_string())
            };
            let start = match perf.model_was_loaded {
                Some(true) => " · 🔥 warm start",
                Some(false) => " · 🧊 cold start",
                None => "",
            };
            lines.push(Line::from(format!(
                "  Load: {}{}",
                ms(perf.load_duration_ms),
                start
            )));
            lines.push(Line::from(format!(
                "  Prefill: {}",
                ms(perf.prompt_eval_duration_ms)
            )));
            lines.push(Line::from(format!(
                "  Generation: {}",
                ms(perf.eval_duration_ms)
            )));
            lines.push(Line::from(format!(
                "  Total: {}",
                ms(perf.total_duration_ms)
            )));
            if let Some(tps) = last_msg.and_then(|m| m.tokens_per_second) {
                // Matches the integer-precision convention used by the
                // header and per-message footer for tokens/sec.
                lines.push(Line::from(format!("  Throughput: {tps:.0} tok/s")));
            }
        }
        None => {
            lines.push(Line::from(Span::styled(
                "  No performance metrics yet — send a message to see stats.",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            "[Esc]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Close", Style::default().fg(Color::White)),
    ]));

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Model Info ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the Provider Switch dialog (Ctrl+W): pick a locally-installed
/// Ollama model and switch the active provider to it without restarting.
fn render_provider_switch(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Switch to a locally-installed Ollama model (↑/↓ navigate, Enter to switch, Esc to cancel)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    if app.provider_switch_loading {
        lines.push(Line::from(Span::styled(
            "Loading installed Ollama models…",
            Style::default().fg(Color::DarkGray),
        )));
    } else if app.provider_switch_models.is_empty() {
        lines.push(Line::from(Span::styled(
            "No Ollama models installed. Use Ctrl+D to download one first.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (idx, model) in app.provider_switch_models.iter().enumerate() {
            let is_selected = idx == app.provider_switch_selected;
            let prefix = if is_selected { "> " } else { "  " };
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            lines.push(Line::from(Span::styled(format!("{prefix}{model}"), style)));
        }
    }

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Switch Provider ",
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
    if let Some(model) = &app.model_download_confirm_delete {
        render_model_download_confirm_delete(f, model, area);
        return;
    }
    if let Some(model) = &app.model_download_deleting {
        render_model_download_deleting(f, model, area);
        return;
    }
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
            "[Del]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Delete installed  ", Style::default().fg(Color::White)),
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

/// Render the confirmation step before deleting a locally-installed model
/// (Del in the suggestion list).
fn render_model_download_confirm_delete(f: &mut Frame, model: &str, area: Rect) {
    let lines = vec![
        Line::from(vec![Span::styled(
            format!("🗑️ Delete '{}'?", model),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "  This removes the model from disk. You can re-pull it later.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Y/Enter]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Confirm delete  ", Style::default().fg(Color::White)),
            Span::styled(
                "[N/Esc]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Cancel", Style::default().fg(Color::White)),
        ]),
    ];

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(Span::styled(
                    " Confirm Delete ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the brief in-flight state while a delete request is running.
fn render_model_download_deleting(f: &mut Frame, model: &str, area: Rect) {
    let lines = vec![Line::from(vec![Span::styled(
        format!("🗑️ Deleting '{}'…", model),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )])];

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(
                    " Deleting… ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Formats a parameter count for compact display (`7_600_000_000 ->
/// "7.6B"`, `500_000_000 -> "500M"`, `1_500 -> "2K"`) - one significant
/// decimal place above the million mark, none below it, since a Ctrl+G row
/// doesn't have room for (and a user doesn't need) an exact count.
fn format_param_count(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.0}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.0}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Fit label + color against `budget_bytes`, with the context length the
/// estimate assumed appended when known (`"Fits (ctx 8192)"`) - matches
/// `crustly llama-cpp list --best-fit`'s wording (`src/cli/mod.rs`) so the
/// CLI and this dialog never disagree about the same model/hardware pair.
/// `#[cfg(feature = "gguf-management")]`-gated to match
/// `llama_cpp_models::hardware_fit`'s own gate exactly (that module's
/// declaration in `provider/mod.rs` - not just the code inside it - is
/// behind the same feature), calling through to it for the actual
/// three-threshold comparison rather than duplicating it, now that both
/// sides carry the identical gate and can never disagree about which build
/// they're compiled into. Under the `not(...)` build below, the caller's
/// `estimated_memory_bytes` is always `None` anyway (nothing populates
/// `LlamaCppModelSummary`'s estimate fields without this feature), so
/// always returning `None` here matches the real behavior, not just a
/// stub. `None` when either side is unknown - matches `HardwareFit::Unknown`'s
/// "no tag at all" treatment (see `llama_cpp_model_lines`'s caller).
#[cfg(feature = "gguf-management")]
fn llama_cpp_fit_tag(
    estimated_memory_bytes: Option<u64>,
    estimated_memory_context_length: Option<u64>,
    budget_bytes: Option<u64>,
) -> Option<(String, Color)> {
    use crate::llm::provider::llama_cpp_models::HardwareFit;
    let (label, color) = match crate::llm::provider::llama_cpp_models::hardware_fit(
        estimated_memory_bytes,
        budget_bytes,
    ) {
        HardwareFit::Fits => ("Fits", Color::Green),
        HardwareFit::Tight => ("Tight", Color::Yellow),
        HardwareFit::WontFit => ("Won't fit", Color::Red),
        HardwareFit::Unknown => return None,
    };
    let label = match estimated_memory_context_length {
        Some(ctx) => format!("{label} (ctx {ctx})"),
        None => label.to_string(),
    };
    Some((label, color))
}

#[cfg(not(feature = "gguf-management"))]
fn llama_cpp_fit_tag(
    _estimated_memory_bytes: Option<u64>,
    _estimated_memory_context_length: Option<u64>,
    _budget_bytes: Option<u64>,
) -> Option<(String, Color)> {
    None
}

/// Renders one Ctrl+G dialog entry: always a compact main line (using
/// `display_name` when set - previously this dialog always showed the raw
/// filename, which for an Ollama-sourced entry is a meaningless
/// `sha256-<hex>` blob name; a split-GGUF group or mmproj-paired entry
/// already carries a synthesized `display_name` too, from
/// `list_all_local_models`'s M3/M4/M5 merging - this was the one place
/// that data wasn't being shown). When `is_selected`, an additional
/// indented detail line - the "expandable detail line" the source plan
/// describes, expressed as "expands on selection" rather than a new
/// keybinding, reusing the dialog's existing up/down selection state.
///
/// `budget_bytes` (Phase M12) - this machine's detected VRAM+RAM budget, or
/// `None` when hardware detection hasn't completed/found anything yet -
/// appends a color-coded Fits/Tight/Won't fit tag to the main line, next to
/// the quantization label (glanceable per-row, not buried in the
/// selected-only detail line below).
fn llama_cpp_model_lines(
    model: &super::llama_cpp_download::LlamaCppModelSummary,
    is_selected: bool,
    budget_bytes: Option<u64>,
) -> Vec<Line<'static>> {
    let style = if is_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let prefix = if is_selected { "▶ " } else { "  " };

    let mut name = model.display_name.clone().unwrap_or_else(|| {
        model
            .path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    });
    // A paired base model's projector is folded into this entry (its own
    // row was removed by `pair_mmproj_files`); an unpaired projector keeps
    // its own row, labeled so it's not just a mysteriously-named model -
    // same convention as the CLI's `list` printer (`src/cli/mod.rs`).
    if model.mmproj_path.is_some() {
        name.push_str(" (+ mmproj)");
    } else if model.is_mmproj {
        name.push_str(" [mmproj]");
    }

    let size_gb = model.size_bytes as f64 / 1_073_741_824.0;
    let quant = model.quantization_hint.as_deref().unwrap_or("unknown");

    let mut main_line_spans = vec![
        Span::styled(prefix, style),
        Span::styled(format!("{name}  ({size_gb:.2} GB, {quant})"), style),
    ];
    if let Some((label, color)) = llama_cpp_fit_tag(
        model.estimated_memory_bytes,
        model.estimated_memory_context_length,
        budget_bytes,
    ) {
        // When selected, keep the row's own highlight styling for the tag
        // too (bold, same fg/bg) rather than the distinct color - a
        // differently-colored span would punch an inconsistent-looking
        // hole in the selection highlight bar.
        let tag_style = if is_selected {
            style
        } else {
            Style::default().fg(color)
        };
        main_line_spans.push(Span::styled(format!("  [{label}]"), tag_style));
    }
    let mut lines = vec![Line::from(main_line_spans)];

    if is_selected {
        let mut parts = Vec::new();
        if let Some(arch) = &model.architecture {
            parts.push(arch.clone());
        }
        if let Some(params) = model.parameter_count {
            parts.push(format!("{} params", format_param_count(params)));
        }
        if let Some(ctx) = model.context_length {
            parts.push(format!("ctx {ctx}"));
        }
        if let Some(mem) = model.estimated_memory_bytes {
            let gb = mem as f64 / 1_073_741_824.0;
            parts.push(if model.estimated_memory_includes_kv_cache {
                format!("~{gb:.1} GB")
            } else {
                format!("~{gb:.1} GB weights")
            });
        }
        parts.push(
            if model.has_chat_template {
                "chat template"
            } else {
                "no chat template"
            }
            .to_string(),
        );

        if !parts.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("      {}", parts.join(" · ")),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines
}

/// The Ctrl+G dialog's one-line host-info row (Phase M11): "detecting
/// hardware…" while the background task from `App::open_llama_cpp_models`
/// is in flight, then a GPU/RAM summary once it completes, or nothing (an
/// empty line, keeping row positions stable) if detection somehow never
/// got kicked off - not expected via the normal `open_llama_cpp_models`
/// path, but a defensive fallback rather than a panic.
fn llama_cpp_hardware_line(app: &App) -> Line<'static> {
    let style = Style::default().fg(Color::DarkGray);
    if app.llama_cpp_hardware_loading {
        return Line::from(Span::styled("  detecting hardware…", style));
    }
    let Some(hardware) = &app.llama_cpp_hardware else {
        return Line::from("");
    };
    let gpu_part = match (&hardware.gpu_name, hardware.vram_available_bytes) {
        (Some(name), Some(vram)) => {
            format!(
                "🖥 {name} (~{:.1} GB VRAM free)",
                vram as f64 / 1_073_741_824.0
            )
        }
        (Some(name), None) => format!("🖥 {name} (VRAM unknown)"),
        (None, _) => "🖥 CPU-only".to_string(),
    };
    let ram_part = match hardware.system_ram_total_bytes {
        Some(bytes) => format!("RAM ~{:.1} GB", bytes as f64 / 1_073_741_824.0),
        None => "RAM unknown".to_string(),
    };
    Line::from(Span::styled(format!("  {gpu_part} · {ram_part}"), style))
}

/// Render the llama.cpp Local Models dialog (Ctrl+G): pick a locally-present
/// `.gguf` file to switch to, or type a URL/`hf:org/repo/file.gguf`
/// shorthand to download a new one.
fn render_llama_cpp_models(f: &mut Frame, app: &App, area: Rect) {
    if let Some(path) = &app.llama_cpp_confirm_delete {
        render_llama_cpp_confirm_delete(f, path, area);
        return;
    }
    if let Some(path) = &app.llama_cpp_deleting {
        render_llama_cpp_deleting(f, path, area);
        return;
    }
    if let Some(path) = &app.llama_cpp_switching {
        render_llama_cpp_switching(f, path, area);
        return;
    }
    if app.llama_cpp_download_running {
        render_llama_cpp_download_progress(f, app, area);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![Span::styled(
        "⚙️ Local llama.cpp models",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(llama_cpp_hardware_line(app));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  > ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if app.llama_cpp_download_input.is_empty() {
                "type a URL or hf:org/repo/file.gguf to download a new model"
            } else {
                app.llama_cpp_download_input.as_str()
            },
            if app.llama_cpp_download_input.is_empty() {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            },
        ),
    ]));
    lines.push(Line::from(""));

    if app.llama_cpp_loading {
        lines.push(Line::from(Span::styled(
            "Scanning for local models…",
            Style::default().fg(Color::DarkGray),
        )));
    } else if app.llama_cpp_models.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No local .gguf models found. Type a URL/hf: shorthand above and press Enter.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let budget_bytes = app
            .llama_cpp_hardware
            .as_ref()
            .and_then(super::llama_cpp_download::HardwareSummary::budget_bytes);
        for (idx, model) in app.llama_cpp_models.iter().enumerate() {
            lines.extend(llama_cpp_model_lines(
                model,
                idx == app.llama_cpp_selected,
                budget_bytes,
            ));
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
            "[Enter]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " Switch (or Download if typing)  ",
            Style::default().fg(Color::White),
        ),
        Span::styled(
            "[Del]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Delete  ", Style::default().fg(Color::White)),
        Span::styled(
            "[Esc]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Cancel", Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(Span::styled(
        "Note: switching loads the whole model file - this can take a while, unlike Ollama's instant swap.",
        Style::default().fg(Color::DarkGray),
    )));

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " Local Models ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the live progress view of an in-flight `.gguf` download.
fn render_llama_cpp_download_progress(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![Span::styled(
        format!("⚙️ Downloading '{}'", app.llama_cpp_download_input),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));

    let status = app
        .llama_cpp_download_status
        .as_deref()
        .unwrap_or("working…");
    lines.push(Line::from(vec![Span::styled(
        format!("  {}", status),
        Style::default().fg(Color::White),
    )]));

    if let Some(fraction) = app.llama_cpp_download_fraction {
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

/// Render the confirmation step before deleting a local model (Del in the
/// list).
fn render_llama_cpp_confirm_delete(f: &mut Frame, path: &std::path::Path, area: Rect) {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let lines = vec![
        Line::from(vec![Span::styled(
            format!("🗑️ Delete '{}'?", name),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "  This removes the file from disk. You can re-download it later.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Y/Enter]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Confirm delete  ", Style::default().fg(Color::White)),
            Span::styled(
                "[N/Esc]",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Cancel", Style::default().fg(Color::White)),
        ]),
    ];

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(Span::styled(
                    " Confirm Delete ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the brief in-flight state while a delete request is running.
fn render_llama_cpp_deleting(f: &mut Frame, path: &std::path::Path, area: Rect) {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let lines = vec![Line::from(vec![Span::styled(
        format!("🗑️ Deleting '{}'…", name),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )])];

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(
                    " Deleting… ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(widget, area);
}

/// Render the blocking "Loading model…" state while a picked model is being
/// loaded as the active provider (`llama-cpp-2-integration-plan.md` §4.5) -
/// deliberately not instant, unlike Ollama's Ctrl+W swap.
fn render_llama_cpp_switching(f: &mut Frame, path: &std::path::Path, area: Rect) {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let lines = vec![
        Line::from(vec![Span::styled(
            format!("⏳ Loading '{}'…", name),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "  This reads the whole model file into memory - it can take a while for large files.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(
                    " Loading Model… ",
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
        AppMode::ModelInfo => "MODEL INFO",
        AppMode::ProviderSwitch => "SWITCH PROVIDER",
        AppMode::LlamaCppModelPicker => "LOCAL MODELS",
        AppMode::Skills => "/SKILLS",
        AppMode::Mcp => "/MCP",
    };

    let auto_mode = app.auto_mode();
    let auto_mode_label = match auto_mode {
        PlanExecMode::Interactive => "⚙ Interactive",
        PlanExecMode::AutoPlan => "⚡ AutoPlan",
        PlanExecMode::FullAuto => "⚡⚡ FullAuto",
    };

    let status = if let Some(ref error) = app.error_message {
        format!(" [{}] {} │ ERROR: {}", mode_text, auto_mode_label, error)
    } else if app.is_processing {
        format!(" [{}] {} │ Processing...", mode_text, auto_mode_label)
    } else {
        format!(
            " [{}] {} │ Shift+Tab: Auto Mode │ Ctrl+H: Help │ Ctrl+D: Download Model │ Ctrl+G: Local Models │ Ctrl+O: Model Info │ Ctrl+W: Switch Model │ Ctrl+K: Clear │ Ctrl+L: Sessions │ Ctrl+N: New │ Ctrl+C: Quit",
            mode_text, auto_mode_label
        )
    };

    let status_color = if app.error_message.is_some() {
        Color::Red
    } else if app.is_processing {
        Color::Yellow
    } else {
        match auto_mode {
            PlanExecMode::Interactive => Color::Green,
            PlanExecMode::AutoPlan => Color::Yellow,
            PlanExecMode::FullAuto => Color::Red,
        }
    };

    let status_bar =
        Paragraph::new(status).style(Style::default().fg(Color::Black).bg(status_color));

    f.render_widget(status_bar, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::llm::agent::AgentService;
    use crate::llm::provider::{
        LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    };
    use crate::services::ServiceContext;
    use crate::tui::app::DisplayMessage;
    use async_trait::async_trait;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::sync::Arc;

    /// Minimal `Provider` stub - these tests only exercise rendering, never
    /// an actual `complete()`/`stream()` call.
    struct DummyProvider;

    #[async_trait]
    impl Provider for DummyProvider {
        async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse> {
            unimplemented!("rendering tests never call complete()")
        }
        async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream> {
            unimplemented!("rendering tests never call stream()")
        }
        fn name(&self) -> &str {
            "dummy"
        }
        fn default_model(&self) -> &str {
            "dummy-model"
        }
        fn supported_models(&self) -> Vec<String> {
            vec!["dummy-model".to_string()]
        }
        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }
        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.0
        }
    }

    async fn test_app() -> App {
        let db = Database::connect_in_memory().await.expect("in-memory db");
        db.run_migrations().await.expect("run migrations");
        let context = ServiceContext::new(db.pool().clone());
        let agent_service = Arc::new(AgentService::new(Arc::new(DummyProvider), context.clone()));
        App::new(agent_service, context)
    }

    /// Render `app` into a small offscreen buffer and return its text
    /// content as a single string, for substring assertions.
    fn render_to_string(app: &App, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("create terminal");
        terminal.draw(|f| render(f, app)).expect("draw frame");

        let buffer = terminal.backend().buffer();
        let mut out = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                out.push_str(buffer[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    /// Flatten a rendered line's spans into plain text.
    fn line_text(line: &Line<'_>) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    /// Regression: message timestamps are stored in UTC but were formatted
    /// with `%H:%M:%S` directly, so the transcript showed UTC clock time - two
    /// hours off from the user's wall clock in a UTC+2 zone. The header must
    /// render the timestamp in the local timezone.
    #[test]
    fn message_header_timestamp_is_shown_in_local_time() {
        // A fixed UTC instant. What the header must show is this instant
        // converted to whatever local zone the test host is in - computed the
        // same way, so the assertion holds in every timezone (including UTC,
        // where local == UTC and the test still passes).
        let utc = chrono::DateTime::parse_from_rfc3339("2026-07-19T13:07:04Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let expected_local = utc
            .with_timezone(&chrono::Local)
            .format("%H:%M:%S")
            .to_string();

        let msg = DisplayMessage {
            id: uuid::Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "hi".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: utc,
            token_count: None,
            cost: None,
            provider_name: Some("ollama".to_string()),
            perf_metrics: None,
            tokens_per_second: None,
        };

        let header = line_text(&render_message_lines(&msg, "ollama:model")[0]);
        assert!(
            header.contains(&expected_local),
            "header should show local time {expected_local}, got: {header}"
        );

        // If the host is NOT in UTC, the raw UTC clock string must be absent -
        // proving the conversion actually happened rather than coincidentally
        // matching. In UTC hosts local == UTC, so this check is skipped.
        let utc_str = utc.format("%H:%M:%S").to_string();
        if utc_str != expected_local {
            assert!(
                !header.contains(&utc_str),
                "header must not show the raw UTC time {utc_str} when local differs, got: {header}"
            );
        }
    }

    #[tokio::test]
    async fn header_shows_ollama_provider_badge_and_tokens_per_second() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.current_session = Some(crate::db::models::Session {
            id: uuid::Uuid::new_v4(),
            title: Some("Test session".to_string()),
            model: Some("qwen2.5-coder:7b".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            archived_at: None,
            token_count: 0,
            total_cost: 0.0,
            provider: Some("ollama".to_string()),
        });
        app.messages.push(DisplayMessage {
            id: uuid::Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "hi".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: Some(30),
            cost: Some(0.0),
            provider_name: Some("ollama".to_string()),
            perf_metrics: None,
            tokens_per_second: Some(42.0),
        });

        // Wide terminal: the header is a single un-wrapped line, so it must
        // be wide enough to fit session/model/tokens/cost/tok-per-sec.
        let screen = render_to_string(&app, 160, 20);
        assert!(screen.contains("qwen2.5-coder:7b"));
        assert!(screen.contains("🦙"));
        assert!(screen.contains("42 tok/s"));
    }

    #[tokio::test]
    async fn header_omits_tokens_per_second_when_unavailable() {
        // Non-Ollama providers (or providers without perf metrics) must not
        // show a fabricated "0 tok/s".
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        let screen = render_to_string(&app, 100, 20);
        assert!(!screen.contains("tok/s"));
    }

    #[tokio::test]
    async fn status_bar_shows_interactive_by_default() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Interactive"));
    }

    #[tokio::test]
    async fn status_bar_shows_full_auto_when_active() {
        use crate::config::PlanExecMode;

        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.set_auto_mode_state(std::sync::Arc::new(std::sync::Mutex::new(
            PlanExecMode::FullAuto,
        )));

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("FullAuto"));
        // The persistent indicator must survive an error state too - Auto
        // Mode being active should never become invisible.
        app.error_message = Some("boom".to_string());
        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("FullAuto"));
    }

    #[tokio::test]
    async fn skills_view_shows_name_and_description() {
        let mut app = test_app().await;
        app.mode = AppMode::Skills;
        app.skills_list = vec![crate::llm::tools::skill::SkillListing {
            name: "my-skill".to_string(),
            description: Some("Does something cool".to_string()),
            root: std::path::PathBuf::new(),
        }];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("my-skill"));
        assert!(screen.contains("Does something cool"));
    }

    #[tokio::test]
    async fn skills_view_shows_empty_state_message() {
        let mut app = test_app().await;
        app.mode = AppMode::Skills;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("No skills found"));
    }

    #[tokio::test]
    async fn mcp_view_shows_connected_server_with_tool_count() {
        let mut app = test_app().await;
        app.mode = AppMode::Mcp;
        app.mcp_status = vec![crate::mcp::McpServerStatus {
            name: "my-server".to_string(),
            command: "my-mcp-binary".to_string(),
            connected: true,
            tool_count: 5,
            error: None,
        }];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("my-server"));
        assert!(screen.contains("connected, 5 tools"));
    }

    #[tokio::test]
    async fn mcp_view_shows_connection_error() {
        let mut app = test_app().await;
        app.mode = AppMode::Mcp;
        app.mcp_status = vec![crate::mcp::McpServerStatus {
            name: "broken-server".to_string(),
            command: "nonexistent-binary".to_string(),
            connected: false,
            tool_count: 0,
            error: Some("No such file or directory".to_string()),
        }];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("broken-server"));
        assert!(screen.contains("not connected"));
        assert!(screen.contains("No such file or directory"));
    }

    #[tokio::test]
    async fn mcp_view_shows_empty_state_message() {
        let mut app = test_app().await;
        app.mode = AppMode::Mcp;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("No MCP servers configured"));
    }

    #[tokio::test]
    async fn model_download_dialog_shows_prompt_and_suggestions() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_suggestions = vec!["qwen2.5-coder:7b".to_string()];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Download an Ollama model"));
        assert!(screen.contains("qwen2.5-coder:7b"));
    }

    #[tokio::test]
    async fn model_download_progress_shows_status_and_bar() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_input = "qwen2.5-coder:7b".to_string();
        app.model_download_running = true;
        app.model_download_status = Some("pulling abc123".to_string());
        app.model_download_fraction = Some(0.5);

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Downloading 'qwen2.5-coder:7b'"));
        assert!(screen.contains("pulling abc123"));
        assert!(screen.contains("50%"));
    }

    #[tokio::test]
    async fn model_download_confirm_delete_shows_prompt() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_confirm_delete = Some("qwen2.5-coder:7b".to_string());

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Delete 'qwen2.5-coder:7b'?"));
        assert!(screen.contains("Confirm delete"));
    }

    #[tokio::test]
    async fn model_download_deleting_shows_status() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_deleting = Some("qwen2.5-coder:7b".to_string());

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Deleting 'qwen2.5-coder:7b'"));
    }

    #[tokio::test]
    async fn llama_cpp_models_dialog_shows_prompt_and_local_models() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_models = vec![super::super::llama_cpp_download::LlamaCppModelSummary {
            path: std::path::PathBuf::from("/models/qwen2.5-coder-7b-Q4_K_M.gguf"),
            size_bytes: 4_294_967_296,
            quantization_hint: Some("Q4_K_M".to_string()),
            architecture: None,
            parameter_count: None,
            context_length: None,
            has_chat_template: false,
            display_name: None,
            estimated_memory_bytes: None,
            estimated_memory_includes_kv_cache: false,
            estimated_memory_context_length: None,
            is_mmproj: false,
            mmproj_path: None,
        }];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Local llama.cpp models"));
        assert!(screen.contains("qwen2.5-coder-7b-Q4_K_M.gguf"));
        assert!(screen.contains("Q4_K_M"));
    }

    fn llama_cpp_model_fixture(
        name: &str,
    ) -> super::super::llama_cpp_download::LlamaCppModelSummary {
        super::super::llama_cpp_download::LlamaCppModelSummary {
            path: std::path::PathBuf::from(name),
            size_bytes: 4_294_967_296,
            quantization_hint: Some("Q4_K_M".to_string()),
            architecture: Some("qwen2".to_string()),
            parameter_count: Some(7_000_000_000),
            context_length: Some(32768),
            has_chat_template: true,
            display_name: None,
            estimated_memory_bytes: Some(5_200_000_000),
            estimated_memory_includes_kv_cache: true,
            estimated_memory_context_length: None,
            is_mmproj: false,
            mmproj_path: None,
        }
    }

    #[test]
    fn format_param_count_formats_at_the_expected_scale() {
        assert_eq!(format_param_count(7_600_000_000), "7.6B");
        assert_eq!(format_param_count(500_000_000), "500M");
        assert_eq!(format_param_count(1_500), "2K");
        assert_eq!(format_param_count(42), "42");
    }

    #[test]
    fn llama_cpp_model_lines_prefers_display_name_over_the_raw_filename() {
        let mut model = llama_cpp_model_fixture("/models/sha256-abc123.gguf");
        model.display_name = Some("qwen2.5-coder:7b".to_string());

        let lines = llama_cpp_model_lines(&model, false, None);
        let text: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("qwen2.5-coder:7b"));
        assert!(!text.contains("sha256-abc123"));
    }

    #[test]
    fn llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently() {
        let mut paired = llama_cpp_model_fixture("/models/a.gguf");
        paired.mmproj_path = Some(std::path::PathBuf::from("/models/mmproj-a.gguf"));
        let paired_text: String = llama_cpp_model_lines(&paired, false, None)[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(paired_text.contains("(+ mmproj)"));

        let mut unpaired = llama_cpp_model_fixture("/models/mmproj-b.gguf");
        unpaired.is_mmproj = true;
        let unpaired_text: String = llama_cpp_model_lines(&unpaired, false, None)[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(unpaired_text.contains("[mmproj]"));
    }

    #[test]
    fn llama_cpp_model_lines_shows_a_detail_line_only_when_selected() {
        let model = llama_cpp_model_fixture("/models/a.gguf");

        let unselected = llama_cpp_model_lines(&model, false, None);
        assert_eq!(unselected.len(), 1, "no detail line when not selected");

        let selected = llama_cpp_model_lines(&model, true, None);
        assert_eq!(selected.len(), 2, "a detail line appears when selected");
        let detail: String = selected[1]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(detail.contains("qwen2"));
        assert!(detail.contains("7.0B params"));
        assert!(detail.contains("ctx 32768"));
        assert!(detail.contains("4.8 GB")); // 5.2e9 bytes as GiB (1_073_741_824 divisor)
        assert!(detail.contains("chat template"));
    }

    // The three tests below exercise `llama_cpp_fit_tag`'s real threshold
    // logic, which only exists (and is only ever fed a non-`None` estimate
    // in practice) when `gguf-management` is compiled in - see that
    // function's doc comment. `llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown`
    // just below isn't gated, since "no tag when budget is unknown" holds
    // in every build.

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known() {
        let model = llama_cpp_model_fixture("/models/a.gguf"); // 5.2 GB estimate
        let text: String = llama_cpp_model_lines(&model, false, Some(10_000_000_000))[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(text.contains("[Fits]"), "got: {text}");
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget() {
        let model = llama_cpp_model_fixture("/models/a.gguf"); // 5.2 GB estimate
        let text: String = llama_cpp_model_lines(&model, false, Some(1_000_000_000))[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(text.contains("[Won't fit]"), "got: {text}");
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_model_lines_shows_the_context_length_the_estimate_used() {
        let mut model = llama_cpp_model_fixture("/models/a.gguf"); // 5.2 GB estimate
        model.estimated_memory_context_length = Some(8_192);
        let text: String = llama_cpp_model_lines(&model, false, Some(10_000_000_000))[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(text.contains("[Fits (ctx 8192)]"), "got: {text}");
    }

    #[cfg(not(feature = "gguf-management"))]
    #[test]
    fn llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature() {
        // Nothing in a `not(gguf-management)` build ever actually populates
        // `estimated_memory_bytes`, but even if a caller set it directly
        // (as this fixture does), the tag must stay suppressed - matches
        // `llama_cpp_fit_tag`'s `not(feature = ...)` stub always returning
        // `None`.
        let model = llama_cpp_model_fixture("/models/a.gguf"); // 5.2 GB estimate
        let text: String = llama_cpp_model_lines(&model, false, Some(10_000_000_000))[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(!text.contains('['), "got: {text}");
    }

    #[test]
    fn llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown() {
        let model = llama_cpp_model_fixture("/models/a.gguf");
        let text: String = llama_cpp_model_lines(&model, false, None)[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        assert!(!text.contains('['), "got: {text}");
    }

    #[tokio::test]
    async fn llama_cpp_models_dialog_shows_hardware_detecting_state() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_hardware_loading = true;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("detecting hardware"));
    }

    #[tokio::test]
    async fn llama_cpp_models_dialog_shows_hardware_summary_once_detected() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_hardware = Some(super::super::llama_cpp_download::HardwareSummary {
            gpu_name: Some("Test GPU 9000".to_string()),
            vram_available_bytes: Some(21_474_836_480),
            system_ram_total_bytes: Some(34_359_738_368),
        });

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Test GPU 9000"));
        assert!(screen.contains("RAM"));
    }

    #[tokio::test]
    async fn llama_cpp_models_dialog_shows_display_name_not_the_raw_path() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        let mut model = llama_cpp_model_fixture("/models/sha256-deadbeef.gguf");
        model.display_name = Some("qwen2.5-coder:7b".to_string());
        app.llama_cpp_models = vec![model];

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("qwen2.5-coder:7b"));
        assert!(!screen.contains("sha256-deadbeef"));
    }

    #[tokio::test]
    async fn llama_cpp_models_dialog_shows_loading_state() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_loading = true;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Scanning for local models"));
    }

    #[tokio::test]
    async fn llama_cpp_download_progress_shows_status_and_bar() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_download_input = "hf:org/repo/model.gguf".to_string();
        app.llama_cpp_download_running = true;
        app.llama_cpp_download_status = Some("12.0 / 24.0 MB".to_string());
        app.llama_cpp_download_fraction = Some(0.5);

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Downloading 'hf:org/repo/model.gguf'"));
        assert!(screen.contains("12.0 / 24.0 MB"));
        assert!(screen.contains("50%"));
    }

    #[tokio::test]
    async fn llama_cpp_confirm_delete_shows_prompt() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_confirm_delete = Some(std::path::PathBuf::from("/models/qwen.gguf"));

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Delete 'qwen.gguf'?"));
        assert!(screen.contains("Confirm delete"));
    }

    #[tokio::test]
    async fn llama_cpp_switching_shows_loading_message() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_switching = Some(std::path::PathBuf::from("/models/qwen.gguf"));

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Loading 'qwen.gguf'"));
    }

    #[tokio::test]
    async fn chat_input_renders_textarea_contents_and_hint() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.textarea.insert_str("hi");

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("hi"));
        assert!(screen.contains("Enter to send"));
    }

    #[tokio::test]
    async fn model_info_panel_shows_provider_model_and_context_window() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelInfo;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("dummy"));
        assert!(screen.contains("dummy-model"));
        assert!(screen.contains("4096 tokens"));
        assert!(screen.contains("No performance metrics yet"));
    }

    #[tokio::test]
    async fn model_info_panel_shows_last_response_perf_metrics() {
        use crate::llm::provider::PerfMetrics;

        let mut app = test_app().await;
        app.mode = AppMode::ModelInfo;
        app.messages.push(DisplayMessage {
            id: uuid::Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "hi".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: Some(30),
            cost: Some(0.0),
            provider_name: Some("dummy".to_string()),
            perf_metrics: Some(PerfMetrics {
                load_duration_ms: Some(120),
                prompt_eval_duration_ms: Some(45),
                eval_duration_ms: Some(900),
                total_duration_ms: Some(1065),
                model_was_loaded: Some(true),
            }),
            tokens_per_second: Some(43.0),
        });

        // Taller terminal than the other tests: the panel has enough lines
        // (provider/model/context + 5 perf metric rows) that a 20-row
        // frame clips the bottom of the content area.
        let screen = render_to_string(&app, 100, 30);
        assert!(screen.contains("120ms"));
        assert!(screen.contains("warm start"));
        assert!(screen.contains("45ms"));
        assert!(screen.contains("900ms"));
        assert!(screen.contains("1065ms"));
        assert!(screen.contains("43 tok/s"));
    }

    #[tokio::test]
    async fn help_screen_lists_commands_from_every_section() {
        let mut app = test_app().await;
        app.mode = AppMode::Help;

        let screen = render_to_string(&app, 100, 100);
        // GLOBAL COMMANDS
        assert!(screen.contains("Ctrl+C"));
        assert!(screen.contains("Quit application"));
        // CHAT MODE (non-kitty terminal falls back to Alt+Enter)
        assert!(screen.contains("Alt+Enter"));
        assert!(screen.contains("New line in message"));
        // SESSION LIST
        assert!(screen.contains("Navigate through sessions"));
        // PLAN MODE
        assert!(screen.contains("Approve plan and start execution"));
        // FEATURES
        assert!(screen.contains("Syntax Highlighting"));
        // Footer
        assert!(screen.contains("to return to chat"));
    }

    #[tokio::test]
    async fn help_screen_shows_shift_enter_when_kitty_protocol_active() {
        let mut app = test_app().await;
        app.mode = AppMode::Help;
        app.kitty_keyboard_protocol_active = true;

        let screen = render_to_string(&app, 100, 100);
        assert!(screen.contains("Shift+Enter"));
    }

    #[tokio::test]
    async fn chat_shows_pending_plan_banner_only_while_awaiting_approval() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        let mut plan = crate::plan::PlanDocument::new(
            uuid::Uuid::new_v4(),
            "Add login page".to_string(),
            "".to_string(),
        );
        plan.status = crate::plan::PlanStatus::PendingApproval;
        app.current_plan = Some(plan);

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("Plan Pending Approval"));

        app.current_plan.as_mut().unwrap().status = crate::plan::PlanStatus::Approved;
        let screen = render_to_string(&app, 100, 20);
        assert!(!screen.contains("Plan Pending Approval"));
    }

    #[tokio::test]
    async fn chat_message_thinking_block_toggles_between_collapsed_and_expanded() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.messages.push(DisplayMessage {
            id: uuid::Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "answer".to_string(),
            thinking_text: Some("step one\nstep two".to_string()),
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: None,
            tokens_per_second: None,
        });

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("press t to expand"));
        assert!(!screen.contains("step one"));

        app.messages[0].thinking_expanded = true;
        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("press t to collapse"));
        assert!(screen.contains("step one"));
        assert!(screen.contains("step two"));
    }

    #[tokio::test]
    async fn chat_message_perf_footer_reports_cold_and_warm_starts() {
        use crate::llm::provider::PerfMetrics;

        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.messages.push(DisplayMessage {
            id: uuid::Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "hi".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: Some(PerfMetrics {
                load_duration_ms: Some(250),
                prompt_eval_duration_ms: None,
                eval_duration_ms: Some(500),
                total_duration_ms: None,
                model_was_loaded: Some(false),
            }),
            tokens_per_second: Some(12.0),
        });

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("500ms generation"));
        assert!(screen.contains("12 tok/s"));
        assert!(screen.contains("cold start (model loaded in 250ms)"));
    }

    #[tokio::test]
    async fn chat_shows_streaming_response_and_processing_indicator() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.is_processing = true;
        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("is thinking..."));

        app.streaming_response = Some("partial reply".to_string());
        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("[streaming]"));
        assert!(screen.contains("partial reply"));
        // The spinner only shows up before the first token arrives.
        assert!(!screen.contains("is thinking..."));
    }

    #[tokio::test]
    async fn plan_mode_shows_full_document_with_tasks_and_criteria() {
        let mut app = test_app().await;
        app.mode = AppMode::Plan;

        let mut plan = crate::plan::PlanDocument::new(
            uuid::Uuid::new_v4(),
            "Add login page".to_string(),
            "Build a login form".to_string(),
        );
        plan.technical_stack = vec!["React".to_string()];
        plan.test_strategy = "Unit test the form validation".to_string();

        let mut task = crate::plan::PlanTask::new(
            0,
            "Create Login component".to_string(),
            "".to_string(),
            crate::plan::TaskType::Create,
        );
        task.acceptance_criteria = vec!["Renders email and password fields".to_string()];
        plan.tasks.push(task);

        app.current_plan = Some(plan);

        let screen = render_to_string(&app, 100, 40);
        assert!(screen.contains("Add login page"));
        assert!(screen.contains("Build a login form"));
        assert!(screen.contains("React"));
        assert!(screen.contains("Unit test the form validation"));
        assert!(screen.contains("Create Login component"));
        assert!(screen.contains("Renders email and password fields"));
        assert!(screen.contains("Approve"));
    }

    #[tokio::test]
    async fn plan_mode_shows_empty_state_without_a_plan() {
        let mut app = test_app().await;
        app.mode = AppMode::Plan;

        let screen = render_to_string(&app, 100, 20);
        assert!(screen.contains("No active plan"));
    }

    fn test_approval_request(
        tool_input: serde_json::Value,
        capabilities: Vec<String>,
    ) -> crate::tui::events::ToolApprovalRequest {
        let (response_tx, _response_rx) = tokio::sync::mpsc::unbounded_channel();
        crate::tui::events::ToolApprovalRequest {
            request_id: uuid::Uuid::new_v4(),
            tool_name: "write_file".to_string(),
            tool_description: "Writes contents to a file".to_string(),
            tool_input,
            capabilities,
            response_tx,
            requested_at: std::time::Instant::now(),
        }
    }

    #[tokio::test]
    async fn approval_dialog_shows_tool_name_capabilities_and_summarized_params() {
        let mut app = test_app().await;
        app.mode = AppMode::ToolApproval;
        app.pending_approval = Some(test_approval_request(
            serde_json::json!({"path": "src/main.rs", "content": "fn main() {}"}),
            vec!["filesystem-write".to_string()],
        ));

        let screen = render_to_string(&app, 100, 30);
        assert!(screen.contains("write_file"));
        assert!(screen.contains("Writes contents to a file"));
        assert!(screen.contains("filesystem-write"));
        assert!(screen.contains("Parameters"));
        assert!(screen.contains("path"));
        // Compact view never shows the raw JSON block.
        assert!(!screen.contains("Tool Input (JSON)"));
    }

    #[tokio::test]
    async fn approval_dialog_details_view_shows_pretty_printed_json() {
        let mut app = test_app().await;
        app.mode = AppMode::ToolApproval;
        app.show_approval_details = true;
        app.pending_approval = Some(test_approval_request(
            serde_json::json!({"path": "src/main.rs"}),
            vec![],
        ));

        let screen = render_to_string(&app, 100, 30);
        assert!(screen.contains("Tool Input (JSON)"));
        assert!(screen.contains("\"path\""));
    }
}
