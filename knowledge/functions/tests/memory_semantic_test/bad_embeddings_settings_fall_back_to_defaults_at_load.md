---
type: Rust Function
title: bad_embeddings_settings_fall_back_to_defaults_at_load
resource: tests/memory_semantic_test.rs#L361-L379
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
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bad_embeddings_settings_fall_back_to_defaults_at_load()`

# Calls

- [default](../../../functions/src/config/MemoryEmbeddingsConfig/default/default.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)