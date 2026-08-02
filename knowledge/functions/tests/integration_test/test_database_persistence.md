---
type: Rust Function
title: test_database_persistence
resource: tests/integration_test.rs#L426-L463
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/tools/task/FileLock/drop/drop
  - functions/src/services/session/SessionService/get_session
---

# Signature

`async fn test_database_persistence() -> Result<()>`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [drop](../../../functions/src/llm/tools/task/FileLock/drop/drop.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)