---
type: Rust Method
title: candidates
resource: src/db/repository/memory.rs#L423-L440
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/candidates_hide_other_projects_and_inactive_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn candidates( &self, project_id: &ProjectId, now: DateTime<Utc>, ) -> Result<Vec<Memory>>`

# Called by

- [try_recall](../../../../../../functions/src/memory/MemoryStore/try_recall.md)
- [candidates_hide_other_projects_and_inactive_memories](../../../../../../functions/tests/memory_repository_test/candidates_hide_other_projects_and_inactive_memories.md)
- [supersede_keeps_history_and_hides_the_old_memory](../../../../../../functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory.md)