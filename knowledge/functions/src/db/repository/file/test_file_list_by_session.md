---
type: Rust Function
title: test_file_list_by_session
resource: src/db/repository/file.rs#L200-L238
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
  - target: functions/src/db/repository/file/FileRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/Session/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/File/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/count_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_file_list_by_session()`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [new](../../../../../functions/src/db/models/Session/new.md)
- [create](../../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [new](../../../../../functions/src/db/models/File/new.md)
- [create](../../../../../functions/src/db/repository/file/FileRepository/create.md)
- [list_by_session](../../../../../functions/src/db/repository/file/FileRepository/list_by_session.md)
- [count_by_session](../../../../../functions/src/db/repository/file/FileRepository/count_by_session.md)