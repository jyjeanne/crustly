---
type: Rust Method
title: critical
resource: src/tui/error.rs#L144-L146
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn critical(category: ErrorCategory, title: String, message: String) -> Self`

# Calls

- [new](../../../../../functions/src/tui/error/ErrorInfo/new.md)