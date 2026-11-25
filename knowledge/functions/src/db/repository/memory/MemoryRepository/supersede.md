---
type: Rust Method
title: supersede
resource: src/db/repository/memory.rs#L669-L688
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn supersede( &self, old: Uuid, new: Uuid, actor: &str, now: DateTime<Utc>, ) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [consolidate](../../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [approve](../../../../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)
- [supersede_keeps_history_and_hides_the_old_memory](../../../../../../functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory.md)