---
type: Rust Function
title: retry_with_rate_limit
resource: src/llm/provider/retry.rs#L171-L191
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/retry/retry_with_backoff
---

# Signature

`pub async fn retry_with_rate_limit<F, Fut, T>( operation: F, config: &RetryConfig, retry_after: Option<Duration>, ) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,`

# Calls

- [retry_with_backoff](../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)