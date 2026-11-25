---
type: Rust Method
title: handle_start_tag
resource: src/tui/markdown.rs#L83-L91
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/markdown/MarkdownRenderer/start_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/flush_current_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_start_tag(&mut self, tag: Tag)`

# Calls

- [start_code_block](../../../../../functions/src/tui/markdown/MarkdownRenderer/start_code_block.md)
- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)