---
type: Rust Function
title: retry
resource: src/utils/retry.rs#L120-L174
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/utils/retry/RetryableError/retry_after
  called_by:
  - functions/src/utils/retry/test_successful_operation_no_retry
  - functions/src/utils/retry/test_non_retryable_error_fails_immediately
  - functions/src/utils/retry/test_retryable_error_retries
  - functions/src/utils/retry/test_max_attempts_exceeded
  - functions/src/utils/retry/test_no_retry_config
---

# Signature

`pub async fn retry<F, Fut, T, E>( mut operation: F, config: &RetryConfig, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: RetryableError,`

# Calls

- [retry_after](../../../../functions/src/utils/retry/RetryableError/retry_after.md)

# Called by

- [test_successful_operation_no_retry](../../../../functions/src/utils/retry/test_successful_operation_no_retry.md)
- [test_non_retryable_error_fails_immediately](../../../../functions/src/utils/retry/test_non_retryable_error_fails_immediately.md)
- [test_retryable_error_retries](../../../../functions/src/utils/retry/test_retryable_error_retries.md)
- [test_max_attempts_exceeded](../../../../functions/src/utils/retry/test_max_attempts_exceeded.md)
- [test_no_retry_config](../../../../functions/src/utils/retry/test_no_retry_config.md)