---
type: Rust Module
title: session
resource: src/db/repository/session.rs#L1-L334
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-db-models-session
  - external/anyhow-context-result
  - external/chrono-utc
  - external/sqlx-sqlitepool
  - external/uuid-uuid
  - external/super
  - external/crate-db-database
  member_of:
  - packages/crustly
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