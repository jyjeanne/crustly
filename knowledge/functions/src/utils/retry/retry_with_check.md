---
type: Rust Function
title: retry_with_check
resource: src/utils/retry.rs#L179-L226
generated:
  by: okf-rs/0.3.0
---

# Signature

`pub async fn retry_with_check<F, Fut, T, E, C>( mut operation: F, config: &RetryConfig, is_retryable: C, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: std::fmt::Display, C: Fn(&E) -> bool,`