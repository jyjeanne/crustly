---
type: Rust Module
title: retry
resource: src/llm/provider/retry.rs#L1-L421
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-providererror-result
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
  - target: external/rand-rng
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
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

- [RetryConfig](../../../../classes/src/llm/provider/retry/RetryConfig.md)
- [default](../../../../functions/src/llm/provider/retry/RetryConfig/default/default.md)
- [new](../../../../functions/src/llm/provider/retry/RetryConfig/new.md)
- [no_retry](../../../../functions/src/llm/provider/retry/RetryConfig/no_retry.md)
- [aggressive](../../../../functions/src/llm/provider/retry/RetryConfig/aggressive.md)
- [calculate_delay](../../../../functions/src/llm/provider/retry/RetryConfig/calculate_delay.md)
- [retry_with_backoff](../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [retry_with_rate_limit](../../../../functions/src/llm/provider/retry/retry_with_rate_limit.md)
- [extract_retry_after](../../../../functions/src/llm/provider/retry/extract_retry_after.md)
- [parse_retry_seconds](../../../../functions/src/llm/provider/retry/parse_retry_seconds.md)
- [test_retry_config_defaults](../../../../functions/src/llm/provider/retry/test_retry_config_defaults.md)
- [test_retry_config_no_retry](../../../../functions/src/llm/provider/retry/test_retry_config_no_retry.md)
- [test_calculate_delay](../../../../functions/src/llm/provider/retry/test_calculate_delay.md)
- [test_retry_success_immediate](../../../../functions/src/llm/provider/retry/test_retry_success_immediate.md)
- [test_retry_success_after_retries](../../../../functions/src/llm/provider/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../../functions/src/llm/provider/retry/test_retry_max_attempts_exceeded.md)
- [test_retry_non_retryable_error](../../../../functions/src/llm/provider/retry/test_retry_non_retryable_error.md)
- [test_extract_retry_after](../../../../functions/src/llm/provider/retry/test_extract_retry_after.md)
- [test_parse_retry_seconds](../../../../functions/src/llm/provider/retry/test_parse_retry_seconds.md)

# Imports

- `super::error::{ProviderError, Result}`
- `std::future::Future`
- `std::time::Duration`
- `tokio::time::sleep`
- `rand::Rng`
- `regex::Regex`
- `super::*`
- `std::sync::atomic::{AtomicU32, Ordering}`
- `std::sync::Arc`

# Member of

- [crustly](../../../../packages/crustly.md)