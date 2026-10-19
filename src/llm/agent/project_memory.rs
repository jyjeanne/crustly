//! Project memory kept in files: notes the project gives the agent, and a
//! log of the agent's own past work.
//!
//! - [`load_project_notes`] reads the project's notes files (`AGENTS.md`,
//!   `CLAUDE.md`, `.crustly/memory.md` where the `save_memory` tool writes...)
//!   when a request starts, within a token budget, for the system prompt.
//!   They change between requests, so they are read just in time, never
//!   cached.
//! - [`append_progress`] adds an entry to `.crustly/progress.md` after a
//!   request that changed files or ran several tool rounds. The notes point
//!   the model at the log; it reads the log on demand instead of carrying it
//!   in every prompt.
//! - [`summarize_tool_calls`] condenses a request's tool calls into one line,
//!   for that log and for the message history.

use super::context::token_count;
use super::middleware::ToolCallRecord;
use serde_json::Value;
use std::path::Path;

/// Notes files, in the order they are read. A file that does not fit the
/// remaining budget is truncated; the files after it are skipped.
pub const NOTE_FILES: &[&str] = &["CRUSTLY.md", "AGENTS.md", "CLAUDE.md", ".crustly/memory.md"];

/// Where finished work is logged, relative to the working directory.
pub const PROGRESS_FILE: &str = ".crustly/progress.md";

/// The progress log is trimmed, oldest entries first, past this size.
const MAX_PROGRESS_BYTES: usize = 20_000;

/// Below this many tokens of budget left, no further notes file is read.
const MIN_FILE_BUDGET: u32 = 64;

/// Token budget for notes on a model with `window` tokens: an eighth of the
/// window, at most 2,000.
pub fn notes_budget(window: u32) -> u32 {
    (window / 8).min(2000)
}

/// The "Project notes" section for the system prompt, from the notes files
/// in `dir`, or `None` when there are none.
pub fn load_project_notes(dir: &Path, max_tokens: u32) -> Option<String> {
    let mut sections = Vec::new();
    let mut remaining = max_tokens;
    for name in NOTE_FILES {
        if remaining < MIN_FILE_BUDGET {
            break;
        }
        let Ok(text) = std::fs::read_to_string(dir.join(name)) else {
            continue;
        };
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        let (kept, truncated) = take_tokens(text, remaining);
        remaining = remaining.saturating_sub(token_count(&kept));
        let mut section = format!("### {name}\n\n{kept}");
        if truncated {
            section.push_str(&format!("\n\n(Truncated - read {name} for the rest.)"));
        }
        sections.push(section);
    }

    let has_progress = dir.join(PROGRESS_FILE).is_file();
    if sections.is_empty() && !has_progress {
        return None;
    }
    let mut notes =
        String::from("## Project notes\n\nFrom files in the working directory. Follow them.");
    for section in sections {
        notes.push_str("\n\n");
        notes.push_str(&section);
    }
    if has_progress {
        notes.push_str(&format!(
            "\n\nA log of earlier work in this project is in {PROGRESS_FILE}; read it when \
             continuing a task."
        ));
    }
    Some(notes)
}

/// The longest prefix of `text`, cut at a line end, that fits `max_tokens`,
/// and whether anything was cut.
fn take_tokens(text: &str, max_tokens: u32) -> (String, bool) {
    if token_count(text) <= max_tokens {
        return (text.to_string(), false);
    }
    let mut kept = String::new();
    let mut used = 0u32;
    for line in text.lines() {
        let cost = token_count(line) + 1;
        if used + cost > max_tokens {
            break;
        }
        kept.push_str(line);
        kept.push('\n');
        used += cost;
    }
    (kept.trim_end().to_string(), true)
}

/// One request's tool calls as one line: tool, target, and whether it
/// failed, for example `read_file src/main.rs; bash cargo check (failed)`.
/// Long requests keep their first calls and a count of the rest.
pub fn summarize_tool_calls(records: &[ToolCallRecord]) -> Option<String> {
    const MAX_CALLS: usize = 12;
    if records.is_empty() {
        return None;
    }
    let mut parts: Vec<String> = records
        .iter()
        .take(MAX_CALLS)
        .map(|r| {
            let mut part = r.name.clone();
            if let Some(target) = call_target(&r.input) {
                part.push(' ');
                part.push_str(&target);
            }
            if !r.success {
                part.push_str(" (failed)");
            }
            part
        })
        .collect();
    if records.len() > MAX_CALLS {
        parts.push(format!("and {} more", records.len() - MAX_CALLS));
    }
    Some(parts.join("; "))
}

/// What a call acted on: its path, command, pattern or URL, shortened.
fn call_target(input: &Value) -> Option<String> {
    const KEYS: &[&str] = &[
        "path",
        "file_path",
        "command",
        "pattern",
        "url",
        "query",
        "operation",
    ];
    let value = KEYS
        .iter()
        .find_map(|k| input.get(*k).and_then(Value::as_str))?;
    let one_line = value.lines().next().unwrap_or_default().trim();
    let mut target: String = one_line.chars().take(60).collect();
    if one_line.chars().count() > 60 || value.lines().nth(1).is_some() {
        target.push('…');
    }
    Some(target)
}

/// One finished request, for the progress log.
#[derive(Debug, Clone, Copy)]
pub struct ProgressEntry<'a> {
    /// What the user asked.
    pub request: &'a str,
    /// The request's tool calls (see [`summarize_tool_calls`]).
    pub tools: &'a str,
    /// The answer given.
    pub result: &'a str,
}

/// Append `entry` to the progress log in `dir`, creating it if needed and
/// dropping the oldest entries once it grows past its size cap.
pub fn append_progress(dir: &Path, entry: &ProgressEntry) -> std::io::Result<()> {
    let path = dir.join(PROGRESS_FILE);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut log = std::fs::read_to_string(&path).unwrap_or_default();
    if log.trim().is_empty() {
        log = "# Crustly progress log\n\nWork done by the agent in this project, newest last.\n"
            .to_string();
    }
    if !log.ends_with('\n') {
        log.push('\n');
    }
    log.push_str(&format!(
        "\n## {} - {}\n\n- Tools: {}\n- Result: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
        first_line(entry.request, 100),
        entry.tools,
        first_line(entry.result, 200),
    ));

    while log.len() > MAX_PROGRESS_BYTES {
        let Some(first) = log.find("\n## ") else {
            break;
        };
        let Some(second) = log[first + 1..].find("\n## ").map(|i| first + 1 + i) else {
            break;
        };
        log.replace_range(first..second, "");
    }

    std::fs::write(path, log)
}

/// The first non-empty line of `text`, at most `max_chars` long.
fn first_line(text: &str, max_chars: usize) -> String {
    let line = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or_default();
    let mut short: String = line.chars().take(max_chars).collect();
    if line.chars().count() > max_chars {
        short.push('…');
    }
    short
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn record(name: &str, input: Value, success: bool) -> ToolCallRecord {
        ToolCallRecord {
            name: name.to_string(),
            input,
            success,
        }
    }

    #[test]
    fn notes_come_from_the_files_present_in_order() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load_project_notes(dir.path(), 1000).is_none());

        std::fs::write(dir.path().join("CLAUDE.md"), "Use cargo nextest.").unwrap();
        std::fs::write(dir.path().join("AGENTS.md"), "Never push to main.").unwrap();
        let notes = load_project_notes(dir.path(), 1000).unwrap();
        assert!(notes.starts_with("## Project notes"));
        let agents = notes.find("### AGENTS.md").unwrap();
        let claude = notes.find("### CLAUDE.md").unwrap();
        assert!(agents < claude, "AGENTS.md is read before CLAUDE.md");
        assert!(notes.contains("Use cargo nextest."));
        assert!(!notes.contains(PROGRESS_FILE));
    }

    #[test]
    fn notes_are_truncated_to_the_budget_at_line_ends() {
        let dir = tempfile::tempdir().unwrap();
        let long = (0..400)
            .map(|i| format!("Rule number {i}: keep functions small."))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(dir.path().join("AGENTS.md"), &long).unwrap();
        std::fs::write(dir.path().join("CLAUDE.md"), "Never reached.").unwrap();

        let notes = load_project_notes(dir.path(), 200).unwrap();
        assert!(notes.contains("Rule number 0:"));
        assert!(!notes.contains("Rule number 399:"));
        assert!(notes.contains("(Truncated - read AGENTS.md for the rest.)"));
        assert!(token_count(&notes) < 300, "{} tokens", token_count(&notes));
        assert!(
            !notes.contains("Never reached."),
            "files after an exhausted budget are skipped"
        );
    }

    #[test]
    fn notes_point_at_the_progress_log_when_there_is_one() {
        let dir = tempfile::tempdir().unwrap();
        append_progress(
            dir.path(),
            &ProgressEntry {
                request: "fix the bug",
                tools: "edit_file src/lib.rs",
                result: "Fixed.",
            },
        )
        .unwrap();
        let notes = load_project_notes(dir.path(), 1000).unwrap();
        assert!(notes.contains(PROGRESS_FILE));
    }

    #[test]
    fn notes_budget_is_an_eighth_of_the_window_up_to_2000() {
        assert_eq!(notes_budget(8192), 1024);
        assert_eq!(notes_budget(200_000), 2000);
    }

    #[test]
    fn tool_calls_are_summarized_with_their_targets() {
        let summary = summarize_tool_calls(&[
            record("read_file", json!({"path": "src/main.rs"}), true),
            record("bash", json!({"command": "cargo check\nmore"}), false),
            record("plan", json!({"operation": "create"}), true),
        ])
        .unwrap();
        assert_eq!(
            summary,
            "read_file src/main.rs; bash cargo check… (failed); plan create"
        );
        assert_eq!(summarize_tool_calls(&[]), None);
    }

    #[test]
    fn long_requests_keep_the_first_calls_and_count_the_rest() {
        let records: Vec<_> = (0..15)
            .map(|i| record("ls", json!({ "path": format!("d{i}") }), true))
            .collect();
        let summary = summarize_tool_calls(&records).unwrap();
        assert!(summary.ends_with("and 3 more"), "{summary}");
    }

    #[test]
    fn the_progress_log_grows_and_drops_its_oldest_entries() {
        let dir = tempfile::tempdir().unwrap();
        let result = "x".repeat(150);
        for i in 0..200 {
            append_progress(
                dir.path(),
                &ProgressEntry {
                    request: &format!("request {i}\nsecond line"),
                    tools: "edit_file a.rs",
                    result: &result,
                },
            )
            .unwrap();
        }
        let log = std::fs::read_to_string(dir.path().join(PROGRESS_FILE)).unwrap();
        assert!(log.starts_with("# Crustly progress log"));
        assert!(log.len() <= MAX_PROGRESS_BYTES);
        assert!(log.contains("request 199"));
        assert!(!log.contains("request 0\n") && !log.contains(" - request 0 "));
        assert!(!log.contains("second line"), "only the first line is kept");
    }
}
