---
type: Rust Function
title: create_test_service
resource: src/services/file.rs#L176-L188
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
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn create_test_service() -> (FileService, SessionService)`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [new](../../../../functions/src/services/ServiceContext/new.md)
- [new](../../../../functions/src/services/file/FileService/new.md)
- [new](../../../../functions/src/services/session/SessionService/new.md)