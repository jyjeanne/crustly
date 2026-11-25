---
type: Rust Method
title: summary
resource: src/tui/error.rs#L163-L170
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorSeverity/prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorCategory/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/error/test_error_info_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn summary(&self) -> String`

# Calls

- [prefix](../../../../../functions/src/tui/error/ErrorSeverity/prefix.md)
- [name](../../../../../functions/src/tui/error/ErrorCategory/name.md)

# Called by

- [test_error_info_summary](../../../../../functions/src/tui/error/test_error_info_summary.md)