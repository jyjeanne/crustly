---
type: Rust Module
title: compaction
resource: src/db/repository/compaction.rs#L1-L65
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-db-models-compactionrecord
  - external/anyhow-result
  - external/chrono-datetime
  - external/sqlx-sqlitepool
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [CompactionRecordRepository](../../../../classes/src/db/repository/compaction/CompactionRecordRepository.md)
- [new](../../../../functions/src/db/repository/compaction/CompactionRecordRepository/new.md)
- [insert](../../../../functions/src/db/repository/compaction/CompactionRecordRepository/insert.md)
- [list_for_session](../../../../functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session.md)

# Imports

- `crate::db::models::CompactionRecord`
- `anyhow::Result`
- `chrono::DateTime`
- `sqlx::SqlitePool`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)