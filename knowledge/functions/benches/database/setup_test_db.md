---
type: Rust Function
title: setup_test_db
resource: benches/database.rs#L16-L24
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
---

# Signature

`async fn setup_test_db() -> (Database, TempDir)`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)