---
type: Rust Method
title: error
resource: src/tui/error.rs#L139-L141
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/error/ErrorInfo/from-string/from
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/from-str/from
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/test_error_info_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/test_error_info_with_retry
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn error(category: ErrorCategory, title: String, message: String) -> Self`

# Calls

- [new](../../../../../functions/src/tui/error/ErrorInfo/new.md)

# Called by

- [from](../../../../../functions/src/tui/error/ErrorInfo/from-string/from.md)
- [from](../../../../../functions/src/tui/error/ErrorInfo/from-str/from.md)
- [test_error_info_creation](../../../../../functions/src/tui/error/test_error_info_creation.md)
- [test_error_info_with_retry](../../../../../functions/src/tui/error/test_error_info_with_retry.md)