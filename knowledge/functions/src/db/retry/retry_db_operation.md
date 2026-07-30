---
type: Rust Function
title: retry_db_operation
resource: src/db/retry.rs#L100-L161
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/retry/retry_db_anyhow
  - functions/src/db/retry/test_retry_success_immediate
  - functions/src/db/retry/test_retry_success_after_retries
  - functions/src/db/retry/test_retry_max_attempts_exceeded
  - functions/src/db/retry/test_retry_non_retryable_error
---

# Signature

`pub async fn retry_db_operation<F, Fut, T, E>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: std::fmt::Display,`

# Called by

- [retry_db_anyhow](../../../../functions/src/db/retry/retry_db_anyhow.md)
- [test_retry_success_immediate](../../../../functions/src/db/retry/test_retry_success_immediate.md)
- [test_retry_success_after_retries](../../../../functions/src/db/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../../functions/src/db/retry/test_retry_max_attempts_exceeded.md)
- [test_retry_non_retryable_error](../../../../functions/src/db/retry/test_retry_non_retryable_error.md)