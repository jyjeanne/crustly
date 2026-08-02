---
type: Rust Function
title: create_test_db
resource: tests/error_scenarios_test.rs#L88-L92
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
---

# Signature

`async fn create_test_db() -> Result<Database>`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)