---
type: Rust Method
title: set_anchors
resource: src/db/repository/memory.rs#L309-L333
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_automatically
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn set_anchors( &self, id: Uuid, concepts: &[String], anchored_at: DateTime<Utc>, ) -> Result<()>`

# Called by

- [anchor_automatically](../../../../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [set_anchors](../../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [anchors_are_replaced_listed_and_deleted_with_their_memory](../../../../../../functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../../../../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)