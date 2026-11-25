---
type: Rust Method
title: sanitized
resource: src/config/mod.rs#L287-L372
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_pinned_min_importance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_recall_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_every
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_max
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_min_importance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_min_similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_batch_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_query_timeout_ms
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/memory_sanitized_restores_defaults_for_out_of_range_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/memory_sanitized_keeps_valid_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sanitized(mut self) -> Self`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [default_pinned_min_importance](../../../../functions/src/config/default_pinned_min_importance.md)
- [default_recall_max_tokens](../../../../functions/src/config/default_recall_max_tokens.md)
- [default_extraction_every](../../../../functions/src/config/default_extraction_every.md)
- [default_extraction_max](../../../../functions/src/config/default_extraction_max.md)
- [default_extraction_min_importance](../../../../functions/src/config/default_extraction_min_importance.md)
- [default_embed_provider](../../../../functions/src/config/default_embed_provider.md)
- [default_embed_min_similarity](../../../../functions/src/config/default_embed_min_similarity.md)
- [default_embed_batch_size](../../../../functions/src/config/default_embed_batch_size.md)
- [default_embed_query_timeout_ms](../../../../functions/src/config/default_embed_query_timeout_ms.md)

# Called by

- [from_layers](../../../../functions/src/config/Config/from_layers.md)
- [memory_sanitized_restores_defaults_for_out_of_range_values](../../../../functions/src/config/memory_sanitized_restores_defaults_for_out_of_range_values.md)
- [memory_sanitized_keeps_valid_values](../../../../functions/src/config/memory_sanitized_keeps_valid_values.md)
- [new](../../../../functions/src/memory/MemoryStore/new.md)
- [bad_embeddings_settings_fall_back_to_defaults_at_load](../../../../functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load.md)