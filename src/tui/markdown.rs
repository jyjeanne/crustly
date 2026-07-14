//! Markdown Rendering
//!
//! Converts markdown text to styled Ratatui widgets.

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use super::highlight::highlight_code;

/// Render text verbatim, with no markdown interpretation.
///
/// For content the user typed. CommonMark treats `\` as an escape before ASCII
/// punctuation, so running a user's message through the markdown parser silently
/// eats backslashes in ordinary Windows paths: `C:\Users\me\.crustly` renders as
/// `C:\Users\me.crustly`. A user typing a path means the path, not markup.
pub fn parse_plain_text(text: &str) -> Vec<Line<'static>> {
    text.split('\n')
        .map(|line| Line::from(Span::raw(line.to_string())))
        .collect()
}

/// One-pass converter from `pulldown_cmark` events to styled Ratatui lines.
/// The fields are the accumulator state a single pass of the parser needs to
/// carry between events (the in-progress line, whether we're inside a code
/// block, the current heading/list nesting, etc.) - kept together as a
/// struct rather than threaded through free functions as separate `&mut`
/// parameters.
struct MarkdownRenderer {
    lines: Vec<Line<'static>>,
    current_line: Vec<Span<'static>>,
    in_code_block: bool,
    code_language: String,
    code_content: String,
    list_level: u32,
    heading_level: u32,
}

impl MarkdownRenderer {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: Vec::new(),
            in_code_block: false,
            code_language: String::new(),
            code_content: String::new(),
            list_level: 0,
            heading_level: 1,
        }
    }

    fn flush_current_line(&mut self) {
        if !self.current_line.is_empty() {
            self.lines
                .push(Line::from(std::mem::take(&mut self.current_line)));
        }
    }

    fn start_code_block(&mut self, kind: CodeBlockKind) {
        self.in_code_block = true;
        self.code_language = match kind {
            CodeBlockKind::Fenced(lang) => lang.to_string(),
            CodeBlockKind::Indented => String::new(),
        };

        if !self.code_language.is_empty() {
            self.flush_current_line();
            self.lines.push(Line::from(vec![
                Span::styled("╭─ ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.code_language.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ─", Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    fn handle_start_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Heading(level, ..) => self.heading_level = level as u32,
            Tag::CodeBlock(kind) => self.start_code_block(kind),
            Tag::List(_) => self.list_level += 1,
            Tag::BlockQuote => self.flush_current_line(),
            _ => {}
        }
    }

    fn end_heading(&mut self) {
        if self.current_line.is_empty() {
            return;
        }

        let prefix = match self.heading_level {
            1 => "# ",
            2 => "## ",
            3 => "### ",
            _ => "",
        };
        let mut styled_line = vec![Span::styled(
            prefix.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )];
        for span in &mut self.current_line {
            *span = span.clone().style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            );
        }
        styled_line.extend(std::mem::take(&mut self.current_line));
        self.lines.push(Line::from(styled_line));
        self.lines.push(Line::from("")); // Add spacing after heading
    }

    fn end_code_block(&mut self) {
        self.flush_current_line();

        if !self.code_content.is_empty() {
            let highlighted_lines = if !self.code_language.is_empty() {
                highlight_code(&self.code_content, &self.code_language)
            } else {
                highlight_code(&self.code_content, "text")
            };
            self.lines.extend(highlighted_lines);
        }

        if !self.code_language.is_empty() {
            self.lines.push(Line::from(Span::styled(
                "╰────".to_string(),
                Style::default().fg(Color::DarkGray),
            )));
        }

        self.lines.push(Line::from("")); // Add spacing after code block
        self.in_code_block = false;
        self.code_language.clear();
        self.code_content.clear();
    }

    fn end_list(&mut self) {
        self.list_level = self.list_level.saturating_sub(1);
        if self.list_level == 0 {
            self.lines.push(Line::from("")); // Add spacing after list
        }
    }

    fn end_paragraph(&mut self) {
        self.flush_current_line();
        self.lines.push(Line::from("")); // Add spacing after paragraph
    }

    fn handle_end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Heading(..) => self.end_heading(),
            Tag::CodeBlock(_) => self.end_code_block(),
            Tag::List(_) => self.end_list(),
            Tag::Paragraph => self.end_paragraph(),
            Tag::Item => self.flush_current_line(),
            Tag::BlockQuote => self.lines.push(Line::from("")), // Add spacing after blockquote
            _ => {}
        }
    }

    fn handle_text(&mut self, text: pulldown_cmark::CowStr<'_>) {
        let text_str = text.to_string();
        if self.in_code_block {
            // Accumulate code content for syntax highlighting
            self.code_content.push_str(&text_str);
        } else {
            // Regular text - add to current line
            self.current_line
                .push(Span::styled(text_str, Style::default()));
        }
    }

    fn handle_inline_code(&mut self, code: pulldown_cmark::CowStr<'_>) {
        self.current_line.push(Span::styled(
            format!("`{}`", code),
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ));
    }

    fn handle_rule(&mut self) {
        self.flush_current_line();
        self.lines.push(Line::from(Span::styled(
            "────────────────────────────────────────".to_string(),
            Style::default().fg(Color::DarkGray),
        )));
        self.lines.push(Line::from(""));
    }

    /// Drains any content still in progress and trims the trailing blank
    /// lines the spacing rules above leave behind.
    fn finish(mut self) -> Vec<Line<'static>> {
        if !self.current_line.is_empty() {
            self.lines.push(Line::from(self.current_line));
        }
        while self.lines.last().is_some_and(|line| line.spans.is_empty()) {
            self.lines.pop();
        }
        self.lines
    }
}

/// Parse markdown and convert to styled lines for Ratatui
pub fn parse_markdown(markdown: &str) -> Vec<Line<'static>> {
    let mut renderer = MarkdownRenderer::new();

    for event in Parser::new(markdown) {
        match event {
            Event::Start(tag) => renderer.handle_start_tag(tag),
            Event::End(tag) => renderer.handle_end_tag(tag),
            Event::Text(text) => renderer.handle_text(text),
            Event::Code(code) => renderer.handle_inline_code(code),
            Event::HardBreak | Event::SoftBreak => renderer.flush_current_line(),
            Event::Rule => renderer.handle_rule(),
            _ => {}
        }
    }

    renderer.finish()
}

/// Extract the raw content of the last fenced or indented code block in
/// `markdown`, if any. Used by the "copy last response" action to copy just
/// the code rather than the surrounding prose, since that's usually what's
/// wanted. Returns `None` if the message has no code blocks.
pub fn last_code_block(markdown: &str) -> Option<String> {
    let parser = Parser::new(markdown);
    let mut in_code_block = false;
    let mut current = String::new();
    let mut last: Option<String> = None;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                current.clear();
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                last = Some(std::mem::take(&mut current));
            }
            Event::Text(text) if in_code_block => {
                current.push_str(&text);
            }
            _ => {}
        }
    }

    last.map(|s| s.trim_end_matches('\n').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collect the rendered text of every span, so a test can assert on what the
    /// user actually sees.
    fn rendered_text(lines: &[Line<'static>]) -> String {
        lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Regression: chat messages are rendered as CommonMark, where `\` is an
    /// escape character. A Windows path typed or pasted by the user came out with
    /// its backslashes eaten - `D:\Projets\test-crustly` displayed as
    /// `D:Projetstest-crustly` - which looked like the input had mangled the
    /// paste, but was purely a rendering artifact.
    /// The user's own messages must render verbatim. CommonMark escapes `\`
    /// before ASCII punctuation, so parsing them as markdown ate the backslashes
    /// out of ordinary Windows paths - `C:\Users\jerem\.crustly` came out as
    /// `C:\Users\jerem.crustly`, which looked like the paste had been mangled.
    #[test]
    fn plain_text_keeps_windows_path_backslashes() {
        for path in [
            r"D:\Projets\test-crustly\src",
            r"C:\Users\jerem\.crustly\crustly.db",
            r"src\_internal\mod.rs",
            r"C:\temp\-backup",
            r"C:\a\*\b",
        ] {
            let text = rendered_text(&parse_plain_text(path));
            assert_eq!(text, path, "user text must render verbatim");
        }
    }

    #[test]
    fn plain_text_keeps_markdown_syntax_literal() {
        // A user typing `*not italic*` means the asterisks.
        let text = rendered_text(&parse_plain_text("*not italic* and _not this_"));
        assert_eq!(text, "*not italic* and _not this_");
    }

    #[test]
    fn plain_text_preserves_line_structure() {
        let text = rendered_text(&parse_plain_text("line one\nline two"));
        assert_eq!(text, "line one\nline two");
    }

    /// Documents *why* user messages bypass the markdown parser: it really does
    /// eat these backslashes. Assistant replies still go through it, on purpose.
    #[test]
    fn markdown_escapes_backslash_before_punctuation() {
        let text = rendered_text(&parse_markdown(r"C:\Users\jerem\.crustly"));
        assert!(
            !text.contains(r"\.crustly"),
            "expected CommonMark to eat the escape; if this now fails, the \
             plain-text path for user messages may no longer be needed: {text:?}"
        );
    }

    #[test]
    fn test_parse_simple_text() {
        let md = "Hello world";
        let lines = parse_markdown(md);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_parse_heading() {
        let md = "# Heading 1\n\nSome text";
        let lines = parse_markdown(md);
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_parse_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let lines = parse_markdown(md);
        assert!(lines.len() > 2); // Header, code, footer
    }

    #[test]
    fn last_code_block_extracts_fenced_content() {
        let md =
            "Here's a function:\n\n```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n\nDone.";
        assert_eq!(
            last_code_block(md).unwrap(),
            "fn main() {\n    println!(\"hi\");\n}"
        );
    }

    #[test]
    fn last_code_block_returns_the_last_of_multiple_blocks() {
        let md = "```rust\nfirst\n```\n\nsome text\n\n```python\nsecond\n```";
        assert_eq!(last_code_block(md).unwrap(), "second");
    }

    #[test]
    fn last_code_block_returns_none_without_any_code() {
        let md = "Just plain prose, no code here.";
        assert!(last_code_block(md).is_none());
    }

    #[test]
    fn test_parse_inline_code() {
        let md = "Use `cargo build` to compile";
        let lines = parse_markdown(md);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_parse_list() {
        let md = "- Item 1\n- Item 2\n- Item 3";
        let lines = parse_markdown(md);
        assert!(lines.len() >= 3);
    }

    #[test]
    fn test_parse_horizontal_rule() {
        let md = "Before\n\n---\n\nAfter";
        let lines = parse_markdown(md);
        assert!(lines.len() > 2);
    }

    #[test]
    fn test_empty_markdown() {
        let md = "";
        let lines = parse_markdown(md);
        assert!(lines.is_empty() || lines.iter().all(|l| l.spans.is_empty()));
    }
}
