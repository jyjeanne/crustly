---
type: Rust Function
title: topic_vector
resource: tests/memory_semantic_test.rs#L21-L35
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_semantic_test/FakeEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn topic_vector(text: &str) -> Vec<f32>`

# Calls

- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [embed](../../../functions/tests/memory_semantic_test/FakeEmbedder/embedder/embed.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)