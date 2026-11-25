---
type: Rust Function
title: the_embeddings_settings_build_the_right_provider_or_none
resource: tests/memory_semantic_test.rs#L329-L358
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/MemoryEmbeddingsConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn the_embeddings_settings_build_the_right_provider_or_none()`

# Calls

- [default](../../../functions/src/config/MemoryEmbeddingsConfig/default/default.md)
- [embedder_from_config](../../../functions/src/memory/embedder_from_config.md)