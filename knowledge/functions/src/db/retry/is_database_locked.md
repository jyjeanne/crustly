---
type: Rust Function
title: is_database_locked
resource: src/db/retry.rs#L72-L80
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/retry/retry_db_sqlx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/test_is_database_locked
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_database_locked(err: &sqlx::Error) -> bool`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [retry_db_sqlx](../../../../functions/src/db/retry/retry_db_sqlx.md)
- [test_is_database_locked](../../../../functions/src/db/retry/test_is_database_locked.md)