---
type: Rust Function
title: test_retry_non_retryable_error
resource: src/llm/provider/retry.rs#L373-L395
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_retry_non_retryable_error()`

# Calls

- [retry_with_backoff](../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)