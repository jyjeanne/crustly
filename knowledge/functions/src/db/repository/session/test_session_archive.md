---
type: Rust Function
title: test_session_archive
resource: src/db/repository/session.rs#L302-L333
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/db/repository/session/SessionRepository/archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/unarchive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/Session/is_archived
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_session_archive()`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [new](../../../../../functions/src/db/models/Session/new.md)
- [create](../../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [archive](../../../../../functions/src/db/repository/session/SessionRepository/archive.md)
- [find_by_id](../../../../../functions/src/db/repository/session/SessionRepository/find_by_id.md)
- [unarchive](../../../../../functions/src/db/repository/session/SessionRepository/unarchive.md)
- [is_archived](../../../../../functions/src/db/models/Session/is_archived.md)