---
type: Rust Method
title: anchors_of
resource: src/db/repository/memory.rs#L336-L351
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/timestamp
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn anchors_of(&self, id: Uuid) -> Result<Vec<(String, DateTime<Utc>)>>`

# Calls

- [timestamp](../../../../../../functions/src/db/repository/memory/timestamp.md)

# Called by

- [anchors](../../../../../../functions/src/memory/anchors/MemoryStore/anchors.md)
- [anchors_are_replaced_listed_and_deleted_with_their_memory](../../../../../../functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../../../../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)