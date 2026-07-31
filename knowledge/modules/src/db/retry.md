---
type: Rust Module
title: retry
resource: src/db/retry.rs#L1-L395
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/std-future-future
  - external/std-time-duration
  - external/tokio-time-sleep
  - external/super
  - external/sqlx-error
  - external/std-sync-atomic-atomicu32-ordering
  - external/std-sync-arc
  member_of:
  - packages/crustly
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