---
type: Rust Module
title: retry
resource: src/utils/retry.rs#L1-L424
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/std-future-future
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-time-sleep
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rand-rng
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicu32-ordering
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [RetryableError](../../../interfaces/src/utils/retry/RetryableError.md)
- [retry_after](../../../functions/src/utils/retry/RetryableError/retry_after.md)
- [RetryConfig](../../../classes/src/utils/retry/RetryConfig.md)
- [default](../../../functions/src/utils/retry/RetryConfig/default/default.md)
- [database](../../../functions/src/utils/retry/RetryConfig/database.md)
- [database_aggressive](../../../functions/src/utils/retry/RetryConfig/database_aggressive.md)
- [api](../../../functions/src/utils/retry/RetryConfig/api.md)
- [api_aggressive](../../../functions/src/utils/retry/RetryConfig/api_aggressive.md)
- [no_retry](../../../functions/src/utils/retry/RetryConfig/no_retry.md)
- [calculate_delay](../../../functions/src/utils/retry/RetryConfig/calculate_delay.md)
- [retry](../../../functions/src/utils/retry/retry.md)
- [retry_with_check](../../../functions/src/utils/retry/retry_with_check.md)
- [TestError](../../../classes/src/utils/retry/TestError.md)
- [fmt](../../../functions/src/utils/retry/TestError/std-fmt-display/fmt.md)
- [is_retryable](../../../functions/src/utils/retry/TestError/retryableerror/is_retryable.md)
- [test_successful_operation_no_retry](../../../functions/src/utils/retry/test_successful_operation_no_retry.md)
- [test_non_retryable_error_fails_immediately](../../../functions/src/utils/retry/test_non_retryable_error_fails_immediately.md)
- [test_retryable_error_retries](../../../functions/src/utils/retry/test_retryable_error_retries.md)
- [test_max_attempts_exceeded](../../../functions/src/utils/retry/test_max_attempts_exceeded.md)
- [test_no_retry_config](../../../functions/src/utils/retry/test_no_retry_config.md)
- [test_calculate_delay_exponential](../../../functions/src/utils/retry/test_calculate_delay_exponential.md)
- [test_calculate_delay_capped](../../../functions/src/utils/retry/test_calculate_delay_capped.md)
- [test_preset_configs](../../../functions/src/utils/retry/test_preset_configs.md)

# Imports

- `std::future::Future`
- `std::time::Duration`
- `tokio::time::sleep`
- `rand::Rng`
- `super::*`
- `std::sync::atomic::{AtomicU32, Ordering}`
- `std::sync::Arc`

# Member of

- [crustly](../../../packages/crustly.md)