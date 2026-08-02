---
type: Rust Function
title: highlight_code
resource: src/tui/highlight.rs#L70-L133
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/highlight/find_syntax
  - functions/src/tui/highlight/get_theme
  - functions/src/tui/highlight/syntect_style_to_ratatui
  called_by:
  - functions/src/tui/highlight/test_highlight_rust
  - functions/src/tui/highlight/test_highlight_python
  - functions/src/tui/highlight/test_highlight_javascript
  - functions/src/tui/highlight/test_highlight_unknown_language
  - functions/src/tui/highlight/test_empty_code
  - functions/src/tui/highlight/test_code_with_special_characters
  - functions/src/tui/markdown/MarkdownRenderer/end_code_block
---

# Signature

`pub fn highlight_code(code: &str, language: &str) -> Vec<Line<'static>>`

# Calls

- [find_syntax](../../../../functions/src/tui/highlight/find_syntax.md)
- [get_theme](../../../../functions/src/tui/highlight/get_theme.md)
- [syntect_style_to_ratatui](../../../../functions/src/tui/highlight/syntect_style_to_ratatui.md)

# Called by

- [test_highlight_rust](../../../../functions/src/tui/highlight/test_highlight_rust.md)
- [test_highlight_python](../../../../functions/src/tui/highlight/test_highlight_python.md)
- [test_highlight_javascript](../../../../functions/src/tui/highlight/test_highlight_javascript.md)
- [test_highlight_unknown_language](../../../../functions/src/tui/highlight/test_highlight_unknown_language.md)
- [test_empty_code](../../../../functions/src/tui/highlight/test_empty_code.md)
- [test_code_with_special_characters](../../../../functions/src/tui/highlight/test_code_with_special_characters.md)
- [end_code_block](../../../../functions/src/tui/markdown/MarkdownRenderer/end_code_block.md)