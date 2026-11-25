---
type: Rust Function
title: create_then_update_survives_a_file_backed_wal_pool
resource: src/services/message.rs#L244-L274
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn create_then_update_survives_a_file_backed_wal_pool()`

# Calls

- [connect](../../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [new](../../../../functions/src/services/message/MessageService/new.md)
- [new](../../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)