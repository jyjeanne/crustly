---
type: Rust Function
title: setup_test_db
resource: benches/database.rs#L16-L24
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn setup_test_db() -> (Database, TempDir)`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)