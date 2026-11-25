---
type: Rust Function
title: test_retry_success_immediate
resource: src/db/retry.rs#L291-L314
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/retry/DbRetryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/retry_db_operation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_retry_success_immediate()`

# Calls

- [default](../../../../functions/src/db/retry/DbRetryConfig/default/default.md)
- [retry_db_operation](../../../../functions/src/db/retry/retry_db_operation.md)