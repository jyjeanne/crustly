---
type: Rust Function
title: default_embed_min_similarity
resource: src/config/mod.rs#L254-L256
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/MemoryEmbeddingsConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_embed_min_similarity() -> f32`

# Called by

- [default](../../../functions/src/config/MemoryEmbeddingsConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)