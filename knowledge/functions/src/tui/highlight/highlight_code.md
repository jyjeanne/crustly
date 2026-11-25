---
type: Rust Function
title: highlight_code
resource: src/tui/highlight.rs#L70-L133
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/highlight/find_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/get_theme
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/syntect_style_to_ratatui
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/highlight/test_highlight_rust
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/test_highlight_python
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/test_highlight_javascript
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/test_highlight_unknown_language
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/test_empty_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/test_code_with_special_characters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_code_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn highlight_code(code: &str, language: &str) -> Vec<Line<'static>>`

# Calls

- [find_syntax](../../../../functions/src/tui/highlight/find_syntax.md)
- [get_theme](../../../../functions/src/tui/highlight/get_theme.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [syntect_style_to_ratatui](../../../../functions/src/tui/highlight/syntect_style_to_ratatui.md)

# Called by

- [test_highlight_rust](../../../../functions/src/tui/highlight/test_highlight_rust.md)
- [test_highlight_python](../../../../functions/src/tui/highlight/test_highlight_python.md)
- [test_highlight_javascript](../../../../functions/src/tui/highlight/test_highlight_javascript.md)
- [test_highlight_unknown_language](../../../../functions/src/tui/highlight/test_highlight_unknown_language.md)
- [test_empty_code](../../../../functions/src/tui/highlight/test_empty_code.md)
- [test_code_with_special_characters](../../../../functions/src/tui/highlight/test_code_with_special_characters.md)
- [end_code_block](../../../../functions/src/tui/markdown/MarkdownRenderer/end_code_block.md)