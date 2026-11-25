---
type: Rust Function
title: is_language_supported
resource: src/tui/highlight.rs#L145-L147
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/highlight/find_syntax
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/highlight/test_is_language_supported
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_language_supported(language: &str) -> bool`

# Calls

- [find_syntax](../../../../functions/src/tui/highlight/find_syntax.md)

# Called by

- [test_is_language_supported](../../../../functions/src/tui/highlight/test_is_language_supported.md)