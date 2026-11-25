---
type: Rust Function
title: test_session_crud
resource: src/db/repository/session.rs#L250-L299
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/Session/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_session_crud()`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [new](../../../../../functions/src/db/models/Session/new.md)
- [create](../../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [find_by_id](../../../../../functions/src/db/repository/session/SessionRepository/find_by_id.md)
- [update](../../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [delete](../../../../../functions/src/db/repository/session/SessionRepository/delete.md)