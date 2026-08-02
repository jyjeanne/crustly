---
type: Rust Method
title: end_heading
resource: src/tui/markdown.rs#L93-L120
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/markdown/MarkdownRenderer/handle_end_tag
---

# Signature

`fn end_heading(&mut self)`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_end_tag](../../../../../functions/src/tui/markdown/MarkdownRenderer/handle_end_tag.md)