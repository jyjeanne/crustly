---
type: Rust Function
title: bits_per_weight
resource: src/llm/provider/gguf_metadata.rs#L462-L488
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bits_per_weight(quantization: &str) -> Option<f64>`

# Called by

- [estimate_memory_bytes](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes.md)