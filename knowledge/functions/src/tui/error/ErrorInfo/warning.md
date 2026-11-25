---
type: Rust Method
title: warning
resource: src/tui/error.rs#L134-L136
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/error/test_error_info_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn warning(category: ErrorCategory, title: String, message: String) -> Self`

# Calls

- [new](../../../../../functions/src/tui/error/ErrorInfo/new.md)

# Called by

- [test_error_info_summary](../../../../../functions/src/tui/error/test_error_info_summary.md)