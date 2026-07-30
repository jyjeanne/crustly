---
type: Rust Method
title: end_paragraph
resource: src/tui/markdown.rs#L154-L157
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/markdown/MarkdownRenderer/flush_current_line
  called_by:
  - functions/src/tui/markdown/MarkdownRenderer/handle_end_tag
---

# Signature

`fn end_paragraph(&mut self)`

# Calls

- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [handle_end_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)