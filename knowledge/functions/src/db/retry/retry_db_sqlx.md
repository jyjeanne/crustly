---
type: Rust Function
title: retry_db_sqlx
resource: src/db/retry.rs#L175-L233
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/retry/is_database_locked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/DbRetryConfig/calculate_delay
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn retry_db_sqlx<F, Fut, T>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, sqlx::Error> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, sqlx::Error>>,`

# Calls

- [is_database_locked](../../../../functions/src/db/retry/is_database_locked.md)
- [calculate_delay](../../../../functions/src/db/retry/DbRetryConfig/calculate_delay.md)

# Called by

- [insert](../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)