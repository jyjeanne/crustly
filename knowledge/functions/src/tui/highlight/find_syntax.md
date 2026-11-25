---
type: Rust Function
title: find_syntax
resource: src/tui/highlight.rs#L51-L65
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/highlight/highlight_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/is_language_supported
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_syntax(language: &str) -> Option<&'static SyntaxReference>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [highlight_code](../../../../functions/src/tui/highlight/highlight_code.md)
- [is_language_supported](../../../../functions/src/tui/highlight/is_language_supported.md)