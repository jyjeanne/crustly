---
type: Rust Method
title: handle_rule
resource: src/tui/markdown.rs#L193-L200
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/markdown/MarkdownRenderer/flush_current_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_rule(&mut self)`

# Calls

- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)