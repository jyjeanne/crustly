---
type: Rust Method
title: list
resource: src/memory/mod.rs#L673-L681
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/list_for_project
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/episodes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn list( &self, project: &ProjectIdentity, include_inactive: bool, ) -> Result<Vec<Memory>>`

# Calls

- [list_for_project](../../../../functions/src/db/repository/memory/MemoryRepository/list_for_project.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [episodes](../../../../functions/src/tui/app/episodes.md)
- [three_episodes_become_one_lesson_with_its_sources](../../../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [a_candidate_with_an_unknown_scope_is_dropped](../../../../functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)