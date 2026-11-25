---
type: Rust Function
title: test_retry_success_after_retries
resource: src/db/retry.rs#L317-L344
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/retry/DbRetryConfig/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/retry_db_operation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_retry_success_after_retries()`

# Calls

- [new](../../../../functions/src/db/retry/DbRetryConfig/new.md)
- [retry_db_operation](../../../../functions/src/db/retry/retry_db_operation.md)