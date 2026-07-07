//! Dialog components for TUI.
//!
//! Provides permission approval dialogs, progress panels, and recovery prompts.

use crate::plan::{AutoRunMode, PlanModeState, PlanTask, TaskStatus};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Render the auto-execution progress panel.
///
/// Shows task status as:
///   [1/5] ✓ task title
///   [2/5] ⚡ running...
///   [3/5] ○ pending
pub fn render_auto_exec_progress(
    f: &mut Frame,
    area: Rect,
    state: &PlanModeState,
    tasks: &[PlanTask],
) {
    let (plan_id, task_index, total) = match state {
        PlanModeState::AutoExecuting { plan_id, task_index, total, .. } => {
            (plan_id, *task_index, *total)
        }
        PlanModeState::Executing { plan_id, task_index, total } => {
            (plan_id, *task_index, *total)
        }
        _ => return,
    };

    let block = Block::default()
        .title(" Plan Progress ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, task) in tasks.iter().enumerate() {
        let (icon, color) = if i < task_index {
            ("✓", Color::Green)
        } else if i == task_index {
            ("⚡", Color::Yellow)
        } else {
            ("○", Color::DarkGray)
        };

        let label = if i == task_index {
            format!("[{}/{}] {} running...", i + 1, total, icon)
        } else if i < task_index {
            format!("[{}/{}] {} {}", i + 1, total, icon, task.title)
        } else {
            format!("[{}/{}] {} {}", i + 1, total, icon, task.title)
        };

        lines.push(Line::from(Span::styled(
            label,
            Style::default().fg(color),
        )));
    }

    let para = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

/// Render the crash-recovery dialog.
///
/// Shown at session start when a plan was interrupted mid-execution.
pub fn render_crash_recovery_dialog(
    f: &mut Frame,
    area: Rect,
    plan_title: &str,
    resume_at: usize,
    total: usize,
) {
    // Center a fixed-size popup
    let popup_area = centered_rect(60, 40, area);
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" ⚠ Plan Interrupted ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Plan: {}", plan_title),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!(
            "This plan was interrupted at task {}/{} ({} tasks remaining).",
            resume_at + 1,
            total,
            total.saturating_sub(resume_at)
        )),
        Line::from(""),
        Line::from("Completed tasks will NOT be re-executed."),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "  [R] Resume   [A] Abort plan   [Esc] Dismiss",
            Style::default().fg(Color::Cyan),
        )),
    ];

    let para = Paragraph::new(text)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

/// Render the permission denial notification (shown when a policy blocks a tool).
pub fn render_policy_denial(f: &mut Frame, area: Rect, tool_name: &str, reason: &str) {
    let popup_area = centered_rect(70, 30, area);
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" 🚫 Tool Blocked by Policy ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let text = vec![
        Line::from(""),
        Line::from(format!("Tool: {}", tool_name)),
        Line::from(format!("Reason: {}", reason)),
        Line::from(""),
        Line::from(Span::styled(
            "The LLM has been informed. Execution continues.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let para = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

/// Return a centered Rect within `r` with the given percentage dimensions.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
