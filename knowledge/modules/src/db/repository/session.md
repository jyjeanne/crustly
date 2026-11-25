---
type: Rust Module
title: session
resource: src/db/repository/session.rs#L1-L334
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-db-models-session
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-utc
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
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SessionListOptions](../../../../classes/src/db/repository/session/SessionListOptions.md)
- [SessionRepository](../../../../classes/src/db/repository/session/SessionRepository.md)
- [new](../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [find_by_id](../../../../functions/src/db/repository/session/SessionRepository/find_by_id.md)
- [create](../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [update](../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [delete](../../../../functions/src/db/repository/session/SessionRepository/delete.md)
- [list](../../../../functions/src/db/repository/session/SessionRepository/list.md)
- [list_active](../../../../functions/src/db/repository/session/SessionRepository/list_active.md)
- [list_archived](../../../../functions/src/db/repository/session/SessionRepository/list_archived.md)
- [archive](../../../../functions/src/db/repository/session/SessionRepository/archive.md)
- [unarchive](../../../../functions/src/db/repository/session/SessionRepository/unarchive.md)
- [update_stats](../../../../functions/src/db/repository/session/SessionRepository/update_stats.md)
- [count](../../../../functions/src/db/repository/session/SessionRepository/count.md)
- [test_session_crud](../../../../functions/src/db/repository/session/test_session_crud.md)
- [test_session_archive](../../../../functions/src/db/repository/session/test_session_archive.md)

# Imports

- `crate::db::models::Session`
- `anyhow::{Context, Result}`
- `chrono::Utc`
- `sqlx::SqlitePool`
- `uuid::Uuid`
- `super::*`
- `crate::db::Database`

# Member of

- [crustly](../../../../packages/crustly.md)