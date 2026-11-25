---
type: Rust Function
title: retry
resource: src/utils/retry.rs#L120-L174
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/utils/retry/RetryableError/retry_after
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/utils/retry/test_successful_operation_no_retry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/retry/test_non_retryable_error_fails_immediately
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/retry/test_retryable_error_retries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/retry/test_max_attempts_exceeded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/retry/test_no_retry_config
    resolved_by: tree-sitter
    confidence: exact
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