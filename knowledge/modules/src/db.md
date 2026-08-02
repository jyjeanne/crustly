---
type: Rust Module
title: db
resource: src/db/mod.rs#L1-L440
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/pub-use-models
  - external/pub-use-repository
  - external/pub-use-retry-retry-db-anyhow-retry-db-operation-retry-db-sqlx-dbretryconfig
  - external/anyhow-context-result
  - external/sqlx-sqlite-sqlitepooloptions-sqlitepool
  - external/std-path-path
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [Database](../../classes/src/db/Database.md)
- [connect](../../functions/src/db/Database/connect.md)
- [connect_in_memory](../../functions/src/db/Database/connect_in_memory.md)
- [pool](../../functions/src/db/Database/pool.md)
- [is_connected](../../functions/src/db/Database/is_connected.md)
- [run_migrations](../../functions/src/db/Database/run_migrations.md)
- [close](../../functions/src/db/Database/close.md)
- [PoolExt](../../interfaces/src/db/PoolExt.md)
- [connect_file](../../functions/src/db/SqlitePool/poolext/connect_file.md)
- [connect_in_memory](../../functions/src/db/SqlitePool/poolext/connect_in_memory.md)
- [is_connected](../../functions/src/db/SqlitePool/poolext/is_connected.md)
- [test_connect_in_memory](../../functions/src/db/test_connect_in_memory.md)
- [test_pool_connect_in_memory](../../functions/src/db/test_pool_connect_in_memory.md)
- [tilde_in_the_database_path_is_expanded_to_home](../../functions/src/db/tilde_in_the_database_path_is_expanded_to_home.md)
- [foreign_keys_are_enforced](../../functions/src/db/foreign_keys_are_enforced.md)
- [deleting_a_session_cascades_to_its_messages](../../functions/src/db/deleting_a_session_cascades_to_its_messages.md)
- [migrating_from_pre_modernization_schema_preserves_existing_messages](../../functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages.md)

# Imports

- `pub use models::*`
- `pub use repository::*`
- `pub use retry::{retry_db_anyhow, retry_db_operation, retry_db_sqlx, DbRetryConfig}`
- `anyhow::{Context, Result}`
- `sqlx::{sqlite::SqlitePoolOptions, SqlitePool}`
- `std::path::Path`
- `super::*`

# Member of

- [crustly](../../packages/crustly.md)