---
type: Rust Function
title: retry_db_sqlx
resource: src/db/retry.rs#L175-L233
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/retry/is_database_locked
---

# Signature

`pub async fn retry_db_sqlx<F, Fut, T>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, sqlx::Error> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, sqlx::Error>>,`

# Calls

- [is_database_locked](../../../../functions/src/db/retry/is_database_locked.md)