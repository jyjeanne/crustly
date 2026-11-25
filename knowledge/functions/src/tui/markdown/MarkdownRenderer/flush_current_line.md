---
type: Rust Method
title: flush_current_line
resource: src/tui/markdown.rs#L54-L59
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/markdown/MarkdownRenderer/start_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/handle_start_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_paragraph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/handle_end_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/handle_rule
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn flush_current_line(&mut self)`

# Called by

- [start_code_block](../../../../../functions/src/tui/markdown/MarkdownRenderer/start_code_block.md)
- [handle_start_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_start_tag.md)
- [end_code_block](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_code_block.md)
- [end_paragraph](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_paragraph.md)
- [handle_end_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)
- [handle_rule](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_rule.md)
- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)