---
type: Rust Method
title: start_code_block
resource: src/tui/markdown.rs#L61-L81
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/markdown/MarkdownRenderer/flush_current_line
  called_by:
  - functions/src/tui/markdown/MarkdownRenderer/handle_start_tag
---

# Signature

`fn start_code_block(&mut self, kind: CodeBlockKind)`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [flush_current_line](../../../../../functions/src/tui/markdown/MarkdownRenderer/flush_current_line.md)

# Called by

- [handle_start_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_start_tag.md)