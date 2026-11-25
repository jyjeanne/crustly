---
type: Rust Method
title: relink_project
resource: src/db/repository/memory.rs#L727-L757
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/relink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/relinking_moves_a_duplicate_as_archived_and_the_rest_as_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/relinking_a_project_to_itself_is_refused_and_deletes_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn relink_project(&self, old: &ProjectId, new: &ProjectId) -> Result<u64>`

# Called by

- [relink](../../../../../../functions/src/memory/MemoryStore/relink.md)
- [relink_moves_memories_to_the_new_project](../../../../../../functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project.md)
- [relinking_moves_a_duplicate_as_archived_and_the_rest_as_active](../../../../../../functions/tests/memory_review_fixes_test/relinking_moves_a_duplicate_as_archived_and_the_rest_as_active.md)
- [relinking_a_project_to_itself_is_refused_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/relinking_a_project_to_itself_is_refused_and_deletes_nothing.md)