---
type: Rust Method
title: new
resource: src/tui/error.rs#L109-L126
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/error/ErrorInfo/info
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/warning
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/critical
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new( severity: ErrorSeverity, category: ErrorCategory, title: String, message: String, ) -> Self`

# Called by

- [info](../../../../../functions/src/tui/error/ErrorInfo/info.md)
- [warning](../../../../../functions/src/tui/error/ErrorInfo/warning.md)
- [error](../../../../../functions/src/tui/error/ErrorInfo/error.md)
- [critical](../../../../../functions/src/tui/error/ErrorInfo/critical.md)