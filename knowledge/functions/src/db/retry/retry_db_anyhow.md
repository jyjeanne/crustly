---
type: Rust Function
title: retry_db_anyhow
resource: src/db/retry.rs#L164-L172
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/retry/retry_db_operation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn retry_db_anyhow<F, Fut, T>(operation: F, config: &DbRetryConfig) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,`

# Calls

- [retry_db_operation](../../../../functions/src/db/retry/retry_db_operation.md)
- [context](../../../../functions/src/services/ServiceManager/context.md)