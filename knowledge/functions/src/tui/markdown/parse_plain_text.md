---
type: Rust Function
title: parse_plain_text
resource: src/tui/markdown.rs#L19-L23
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/markdown/plain_text_keeps_windows_path_backslashes
  - functions/src/tui/markdown/plain_text_keeps_markdown_syntax_literal
  - functions/src/tui/markdown/plain_text_preserves_line_structure
  - functions/src/tui/render/render_message_lines
---

# Signature

`pub fn parse_plain_text(text: &str) -> Vec<Line<'static>>`

# Called by

- [plain_text_keeps_windows_path_backslashes](../../../../functions/src/tui/markdown/plain_text_keeps_windows_path_backslashes.md)
- [plain_text_keeps_markdown_syntax_literal](../../../../functions/src/tui/markdown/plain_text_keeps_markdown_syntax_literal.md)
- [plain_text_preserves_line_structure](../../../../functions/src/tui/markdown/plain_text_preserves_line_structure.md)
- [render_message_lines](../../../../functions/src/tui/render/render_message_lines.md)