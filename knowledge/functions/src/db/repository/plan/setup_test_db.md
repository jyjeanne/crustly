---
type: Rust Function
title: setup_test_db
resource: src/db/repository/plan.rs#L720-L740
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
---

# Signature

`async fn setup_test_db() -> (Database, SessionRepository, PlanRepository, Session)`

# Calls

- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)