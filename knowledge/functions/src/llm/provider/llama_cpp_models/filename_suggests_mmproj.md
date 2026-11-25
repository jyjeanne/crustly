---
type: Rust Function
title: filename_suggests_mmproj
resource: src/llm/provider/llama_cpp_models.rs#L182-L185
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn filename_suggests_mmproj(filename: &str) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)