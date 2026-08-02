---
type: Rust Method
title: end_code_block
resource: src/tui/markdown.rs#L122-L145
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/markdown/MarkdownRenderer/flush_current_line
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/highlight/highlight_code
  called_by:
  - functions/src/tui/markdown/MarkdownRenderer/handle_end_tag
---

# Signature

`fn end_code_block(&mut self)`

# Calls

- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [highlight_code](../../../../../functions/src/tui/highlight/highlight_code.md)

# Called by

- [handle_end_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)