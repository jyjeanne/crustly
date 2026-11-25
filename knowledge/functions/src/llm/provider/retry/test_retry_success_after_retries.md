---
type: Rust Function
title: test_retry_success_after_retries
resource: src/llm/provider/retry.rs#L318-L345
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/retry/RetryConfig/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_retry_success_after_retries()`

# Calls

- [new](../../../../../functions/src/llm/provider/retry/RetryConfig/new.md)
- [retry_with_backoff](../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)