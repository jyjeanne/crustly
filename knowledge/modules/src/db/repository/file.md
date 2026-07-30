---
type: Rust Module
title: file
resource: src/db/repository/file.rs#L1-L239
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-db-models-file
  - external/anyhow-context-result
  - external/sqlx-sqlitepool
  - external/std-path-path
  - external/uuid-uuid
  - external/super
  - external/crate-db-models-session
  - external/crate-db-repository-sessionrepository
  - external/crate-db-database
  - external/std-path-pathbuf
  member_of:
  - packages/crustly
---

# Contains

- [FileRepository](../../../../classes/src/db/repository/file/FileRepository.md)
- [new](../../../../functions/src/db/repository/file/FileRepository/new.md)
- [find_by_id](../../../../functions/src/db/repository/file/FileRepository/find_by_id.md)
- [find_by_session](../../../../functions/src/db/repository/file/FileRepository/find_by_session.md)
- [find_by_path](../../../../functions/src/db/repository/file/FileRepository/find_by_path.md)
- [create](../../../../functions/src/db/repository/file/FileRepository/create.md)
- [update](../../../../functions/src/db/repository/file/FileRepository/update.md)
- [delete](../../../../functions/src/db/repository/file/FileRepository/delete.md)
- [list_by_session](../../../../functions/src/db/repository/file/FileRepository/list_by_session.md)
- [count_by_session](../../../../functions/src/db/repository/file/FileRepository/count_by_session.md)
- [delete_by_session](../../../../functions/src/db/repository/file/FileRepository/delete_by_session.md)
- [test_file_crud](../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../functions/src/db/repository/file/test_file_list_by_session.md)

# Imports

- `crate::db::models::File`
- `anyhow::{Context, Result}`
- `sqlx::SqlitePool`
- `std::path::Path`
- `uuid::Uuid`
- `super::*`
- `crate::db::models::Session`
- `crate::db::repository::SessionRepository`
- `crate::db::Database`
- `std::path::PathBuf`

# Member of

- [crustly](../../../../packages/crustly.md)