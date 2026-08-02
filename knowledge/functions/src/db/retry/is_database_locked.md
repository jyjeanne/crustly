---
type: Rust Function
title: is_database_locked
resource: src/db/retry.rs#L72-L80
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/db/retry/retry_db_sqlx
---

# Signature

`fn is_database_locked(err: &sqlx::Error) -> bool`

# Called by

- [retry_db_sqlx](../../../../functions/src/db/retry/retry_db_sqlx.md)