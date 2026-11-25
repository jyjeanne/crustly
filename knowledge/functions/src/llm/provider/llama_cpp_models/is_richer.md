---
type: Rust Function
title: is_richer
resource: src/llm/provider/llama_cpp_models.rs#L553-L555
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/merge_by_canonical_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_richer(candidate: &LocalGgufModel, current: &LocalGgufModel) -> bool`

# Called by

- [merge_by_canonical_path](../../../../../functions/src/llm/provider/llama_cpp_models/merge_by_canonical_path.md)