---
type: Rust Module
title: retry
resource: src/db/retry.rs#L1-L395
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-future-future
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-time-sleep
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-error
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

- [DbRetryConfig](../../../classes/src/db/retry/DbRetryConfig.md)
- [default](../../../functions/src/db/retry/DbRetryConfig/default/default.md)
- [new](../../../functions/src/db/retry/DbRetryConfig/new.md)
- [aggressive](../../../functions/src/db/retry/DbRetryConfig/aggressive.md)
- [calculate_delay](../../../functions/src/db/retry/DbRetryConfig/calculate_delay.md)
- [is_database_locked](../../../functions/src/db/retry/is_database_locked.md)
- [retry_db_operation](../../../functions/src/db/retry/retry_db_operation.md)
- [retry_db_anyhow](../../../functions/src/db/retry/retry_db_anyhow.md)
- [retry_db_sqlx](../../../functions/src/db/retry/retry_db_sqlx.md)
- [test_retry_config_defaults](../../../functions/src/db/retry/test_retry_config_defaults.md)
- [test_retry_config_aggressive](../../../functions/src/db/retry/test_retry_config_aggressive.md)
- [test_calculate_delay](../../../functions/src/db/retry/test_calculate_delay.md)
- [test_is_database_locked](../../../functions/src/db/retry/test_is_database_locked.md)
- [test_retry_success_immediate](../../../functions/src/db/retry/test_retry_success_immediate.md)
- [test_retry_success_after_retries](../../../functions/src/db/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../functions/src/db/retry/test_retry_max_attempts_exceeded.md)
- [test_retry_non_retryable_error](../../../functions/src/db/retry/test_retry_non_retryable_error.md)

# Imports

- `anyhow::{Context, Result}`
- `std::future::Future`
- `std::time::Duration`
- `tokio::time::sleep`
- `super::*`
- `sqlx::Error`
- `std::sync::atomic::{AtomicU32, Ordering}`
- `std::sync::Arc`

# Member of

- [crustly](../../../packages/crustly.md)