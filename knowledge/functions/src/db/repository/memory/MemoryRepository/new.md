---
type: Rust Method
title: new
resource: src/db/repository/memory.rs#L42-L44
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/merge_band_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/repo
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/wait_for_vector
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_vectors_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [reject_refuses_a_memory_that_is_not_pending](../../../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [new](../../../../../../functions/src/memory/MemoryStore/new.md)
- [attach_embedder](../../../../../../functions/src/memory/MemoryStore/attach_embedder.md)
- [memory_screen_app](../../../../../../functions/src/tui/app/memory_screen_app.md)
- [anchors_are_replaced_listed_and_deleted_with_their_memory](../../../../../../functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../../../../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)
- [insert_at](../../../../../../functions/tests/memory_consolidate_test/insert_at.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../../../../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [merge_band_vectors](../../../../../../functions/tests/memory_consolidate_test/merge_band_vectors.md)
- [repo_with_projects](../../../../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [repo](../../../../../../functions/tests/memory_review_fixes_test/repo.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [wait_for_vector](../../../../../../functions/tests/memory_semantic_test/wait_for_vector.md)
- [an_approved_memory_is_embedded](../../../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [user_repeats_raise_importance_and_model_repeats_do_not](../../../../../../functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not.md)
- [setup](../../../../../../functions/tests/memory_vectors_test/setup.md)