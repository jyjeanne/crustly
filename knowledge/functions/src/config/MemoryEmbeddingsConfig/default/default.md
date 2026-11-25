---
type: Rust Method
title: default
resource: src/config/mod.rs#L177-L189
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/default_embed_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_base_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_query_timeout_ms
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_batch_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_embed_min_similarity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/the_embeddings_settings_build_the_right_provider_or_none
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_embed_provider](../../../../../functions/src/config/default_embed_provider.md)
- [default_embed_model](../../../../../functions/src/config/default_embed_model.md)
- [default_embed_base_url](../../../../../functions/src/config/default_embed_base_url.md)
- [default_embed_query_timeout_ms](../../../../../functions/src/config/default_embed_query_timeout_ms.md)
- [default_embed_batch_size](../../../../../functions/src/config/default_embed_batch_size.md)
- [default_embed_min_similarity](../../../../../functions/src/config/default_embed_min_similarity.md)

# Called by

- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)
- [the_embeddings_settings_build_the_right_provider_or_none](../../../../../functions/tests/memory_semantic_test/the_embeddings_settings_build_the_right_provider_or_none.md)
- [bad_embeddings_settings_fall_back_to_defaults_at_load](../../../../../functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load.md)