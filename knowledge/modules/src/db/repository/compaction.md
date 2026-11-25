---
type: Rust Module
title: compaction
resource: src/db/repository/compaction.rs#L1-L65
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-db-models-compactionrecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-sqlitepool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
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