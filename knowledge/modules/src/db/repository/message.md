---
type: Rust Module
title: message
resource: src/db/repository/message.rs#L1-L310
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crate-db-models-message
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-sqlitepool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-models-session
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-sessionrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [MessageRepository](../../../../classes/src/db/repository/message/MessageRepository.md)
- [new](../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [find_by_id](../../../../functions/src/db/repository/message/MessageRepository/find_by_id.md)
- [find_by_session](../../../../functions/src/db/repository/message/MessageRepository/find_by_session.md)
- [create](../../../../functions/src/db/repository/message/MessageRepository/create.md)
- [update](../../../../functions/src/db/repository/message/MessageRepository/update.md)
- [delete](../../../../functions/src/db/repository/message/MessageRepository/delete.md)
- [list_by_session](../../../../functions/src/db/repository/message/MessageRepository/list_by_session.md)
- [count_by_session](../../../../functions/src/db/repository/message/MessageRepository/count_by_session.md)
- [get_last_message](../../../../functions/src/db/repository/message/MessageRepository/get_last_message.md)
- [delete_by_session](../../../../functions/src/db/repository/message/MessageRepository/delete_by_session.md)
- [test_message_crud](../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../functions/src/db/repository/message/test_message_list_by_session.md)

# Imports

- `crate::db::models::Message`
- `anyhow::{Context, Result}`
- `sqlx::SqlitePool`
- `uuid::Uuid`
- `super::*`
- `crate::db::models::Session`
- `crate::db::repository::SessionRepository`
- `crate::db::Database`

# Member of

- [crustly](../../../../packages/crustly.md)