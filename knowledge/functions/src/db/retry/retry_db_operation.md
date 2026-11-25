---
type: Rust Function
title: retry_db_operation
resource: src/db/retry.rs#L100-L161
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/DbRetryConfig/calculate_delay
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/retry/retry_db_anyhow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/test_retry_success_immediate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/test_retry_success_after_retries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/test_retry_max_attempts_exceeded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/test_retry_non_retryable_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn retry_db_operation<F, Fut, T, E>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: std::fmt::Display,`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [calculate_delay](../../../../functions/src/db/retry/DbRetryConfig/calculate_delay.md)

# Called by

- [retry_db_anyhow](../../../../functions/src/db/retry/retry_db_anyhow.md)
- [test_retry_success_immediate](../../../../functions/src/db/retry/test_retry_success_immediate.md)
- [test_retry_success_after_retries](../../../../functions/src/db/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../../functions/src/db/retry/test_retry_max_attempts_exceeded.md)
- [test_retry_non_retryable_error](../../../../functions/src/db/retry/test_retry_non_retryable_error.md)