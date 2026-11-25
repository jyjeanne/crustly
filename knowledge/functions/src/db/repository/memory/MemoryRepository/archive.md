---
type: Rust Method
title: archive
resource: src/db/repository/memory.rs#L145-L155
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn archive(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [consolidate](../../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [archive_near_duplicates](../../../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../../../../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)