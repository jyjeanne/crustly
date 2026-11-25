---
type: Rust Method
title: anchors_for
resource: src/db/repository/memory.rs#L355-L378
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/timestamp
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchored_to
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchored_in_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_locations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
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

`pub async fn anchors_for( &self, project_id: &ProjectId, now: DateTime<Utc>, ) -> Result<Vec<(Uuid, String, DateTime<Utc>)>>`

# Calls

- [timestamp](../../../../../../functions/src/db/repository/memory/timestamp.md)

# Called by

- [anchored_to](../../../../../../functions/src/memory/anchors/MemoryStore/anchored_to.md)
- [anchor_hits](../../../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)
- [anchored_in_file](../../../../../../functions/src/memory/anchors/MemoryStore/anchored_in_file.md)
- [anchor_locations](../../../../../../functions/src/memory/anchors/MemoryStore/anchor_locations.md)
- [drifted](../../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)
- [anchors_are_replaced_listed_and_deleted_with_their_memory](../../../../../../functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../../../../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)