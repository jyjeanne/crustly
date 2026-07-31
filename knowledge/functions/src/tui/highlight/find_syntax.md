---
type: Rust Function
title: find_syntax
resource: src/tui/highlight.rs#L51-L65
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/highlight/highlight_code
  - functions/src/tui/highlight/is_language_supported
---

# Signature

`fn find_syntax(language: &str) -> Option<&'static SyntaxReference>`

# Called by

- [highlight_code](../../../../functions/src/tui/highlight/highlight_code.md)
- [is_language_supported](../../../../functions/src/tui/highlight/is_language_supported.md)