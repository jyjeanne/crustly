---
type: Rust Function
title: fixed_now
resource: tests/memory_repository_test.rs#L13-L15
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_repository_test/new_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/reinforce_raises_importance_up_to_five
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/candidates_hide_other_projects_and_inactive_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fts_search_finds_the_current_words_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/deleted_memory_is_gone_from_storage_and_search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/promote_makes_a_project_memory_global
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fts_search_never_crosses_projects
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fixed_now() -> DateTime<Utc>`

# Called by

- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [reinforce_raises_importance_up_to_five](../../../functions/tests/memory_repository_test/reinforce_raises_importance_up_to_five.md)
- [candidates_hide_other_projects_and_inactive_memories](../../../functions/tests/memory_repository_test/candidates_hide_other_projects_and_inactive_memories.md)
- [fts_search_finds_the_current_words_only](../../../functions/tests/memory_repository_test/fts_search_finds_the_current_words_only.md)
- [deleted_memory_is_gone_from_storage_and_search](../../../functions/tests/memory_repository_test/deleted_memory_is_gone_from_storage_and_search.md)
- [supersede_keeps_history_and_hides_the_old_memory](../../../functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory.md)
- [promote_makes_a_project_memory_global](../../../functions/tests/memory_repository_test/promote_makes_a_project_memory_global.md)
- [relink_moves_memories_to_the_new_project](../../../functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project.md)
- [fts_search_never_crosses_projects](../../../functions/tests/memory_repository_test/fts_search_never_crosses_projects.md)