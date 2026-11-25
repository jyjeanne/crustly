---
type: Rust Function
title: record_event
resource: src/db/repository/memory.rs#L877-L897
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/reinforce
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/update_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/supersede
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/promote
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/delete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn record_event( tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, memory_id: Uuid, event: &str, actor: &str, session_id: Option<Uuid>, now: DateTime<Utc>, ) -> sqlx::Result<()>`

# Called by

- [insert](../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [archive](../../../../../functions/src/db/repository/memory/MemoryRepository/archive.md)
- [approve](../../../../../functions/src/db/repository/memory/MemoryRepository/approve.md)
- [reinforce](../../../../../functions/src/db/repository/memory/MemoryRepository/reinforce.md)
- [update_content](../../../../../functions/src/db/repository/memory/MemoryRepository/update_content.md)
- [supersede](../../../../../functions/src/db/repository/memory/MemoryRepository/supersede.md)
- [promote](../../../../../functions/src/db/repository/memory/MemoryRepository/promote.md)
- [delete](../../../../../functions/src/db/repository/memory/MemoryRepository/delete.md)