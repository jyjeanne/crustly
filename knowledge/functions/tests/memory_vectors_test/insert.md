---
type: Rust Function
title: insert
resource: tests/memory_vectors_test.rs#L34-L55
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
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

`async fn insert(repo: &MemoryRepository, scope: MemoryScope, content: &str) -> Uuid`

# Calls

- [insert](../../../functions/src/db/repository/memory/MemoryRepository/insert.md)

# Called by

- [a_stored_vector_comes_back_for_its_model_only](../../../functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only.md)
- [vectors_of_another_projects_are_never_returned](../../../functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned.md)
- [a_second_put_replaces_the_first_vector](../../../functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector.md)
- [a_vector_whose_size_does_not_match_its_record_is_skipped](../../../functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped.md)
- [forgetting_a_memory_removes_its_vector](../../../functions/tests/memory_vectors_test/forgetting_a_memory_removes_its_vector.md)
- [missing_lists_active_memories_without_a_vector_for_the_model](../../../functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model.md)
- [prune_keeps_only_the_current_model_and_counts_match](../../../functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match.md)