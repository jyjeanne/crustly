---
type: Rust Method
title: put_embedding
resource: src/db/repository/memory.rs#L762-L784
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector/encode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/semantic/embed_and_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/merge_band_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/forgetting_a_memory_removes_its_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn put_embedding( &self, id: Uuid, model: &str, vector: &[f32], now: DateTime<Utc>, ) -> Result<()>`

# Calls

- [encode](../../../../../../functions/src/memory/vector/encode.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [embed_and_store](../../../../../../functions/src/memory/semantic/embed_and_store.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../../../../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [merge_band_vectors](../../../../../../functions/tests/memory_consolidate_test/merge_band_vectors.md)
- [a_stored_vector_comes_back_for_its_model_only](../../../../../../functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only.md)
- [vectors_of_another_projects_are_never_returned](../../../../../../functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned.md)
- [a_second_put_replaces_the_first_vector](../../../../../../functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector.md)
- [a_vector_whose_size_does_not_match_its_record_is_skipped](../../../../../../functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped.md)
- [forgetting_a_memory_removes_its_vector](../../../../../../functions/tests/memory_vectors_test/forgetting_a_memory_removes_its_vector.md)
- [missing_lists_active_memories_without_a_vector_for_the_model](../../../../../../functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model.md)
- [prune_keeps_only_the_current_model_and_counts_match](../../../../../../functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match.md)