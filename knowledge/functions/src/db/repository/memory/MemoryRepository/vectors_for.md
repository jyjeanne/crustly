---
type: Rust Method
title: vectors_for
resource: src/db/repository/memory.rs#L789-L823
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector/decode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/current_index
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/wait_for_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn vectors_for( &self, project_id: &ProjectId, model: &str, now: DateTime<Utc>, ) -> Result<Vec<(Uuid, Vec<f32>)>>`

# Calls

- [decode](../../../../../../functions/src/memory/vector/decode.md)

# Called by

- [consolidate](../../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [current_index](../../../../../../functions/src/memory/semantic/current_index.md)
- [archive_near_duplicates](../../../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [wait_for_vector](../../../../../../functions/tests/memory_semantic_test/wait_for_vector.md)
- [a_stored_vector_comes_back_for_its_model_only](../../../../../../functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only.md)
- [a_second_put_replaces_the_first_vector](../../../../../../functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector.md)
- [a_vector_whose_size_does_not_match_its_record_is_skipped](../../../../../../functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped.md)
- [vectors_of_another_projects_are_never_returned](../../../../../../functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned.md)