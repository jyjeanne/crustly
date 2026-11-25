---
type: Rust Method
title: start_code_block
resource: src/tui/markdown.rs#L61-L81
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/markdown/MarkdownRenderer/flush_current_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/markdown/MarkdownRenderer/handle_start_tag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn start_code_block(&mut self, kind: CodeBlockKind)`

# Calls

- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [handle_start_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_start_tag.md)