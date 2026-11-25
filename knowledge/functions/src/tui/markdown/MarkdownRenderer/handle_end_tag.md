---
type: Rust Method
title: handle_end_tag
resource: src/tui/markdown.rs#L159-L169
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/markdown/MarkdownRenderer/end_heading
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/MarkdownRenderer/end_paragraph
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

`fn handle_end_tag(&mut self, tag: TagEnd)`

# Calls

- [end_heading](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_heading.md)
- [end_code_block](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_code_block.md)
- [end_list](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_list.md)
- [end_paragraph](../../../../../functions/src/tui/markdown/MarkdownRenderer/end_paragraph.md)
- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)