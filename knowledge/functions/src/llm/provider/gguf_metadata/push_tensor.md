---
type: Rust Function
title: push_tensor
resource: src/llm/provider/gguf_metadata.rs#L566-L574
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/tensor_type_mode_is_the_fallback_when_file_type_is_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/tensor_type_mode_breaks_a_tensor_count_tie_deterministically
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parameter_count_sums_across_tensor_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_tensor(buf: &mut Vec<u8>, name: &str, dims: &[u64], ggml_type: u32)`

# Called by

- [tensor_type_mode_is_the_fallback_when_file_type_is_absent](../../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_is_the_fallback_when_file_type_is_absent.md)
- [tensor_type_mode_breaks_a_tensor_count_tie_deterministically](../../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_breaks_a_tensor_count_tie_deterministically.md)
- [parameter_count_sums_across_tensor_dimensions](../../../../../functions/src/llm/provider/gguf_metadata/parameter_count_sums_across_tensor_dimensions.md)