---
type: Rust Function
title: mmproj_core_name
resource: src/llm/provider/llama_cpp_models.rs#L679-L690
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/pair_mmproj_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/mmproj_core_name_reduces_a_base_model_and_its_projector_identically
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mmproj_core_name(filename: &str) -> String`

# Called by

- [pair_mmproj_files](../../../../../functions/src/llm/provider/llama_cpp_models/pair_mmproj_files.md)
- [mmproj_core_name_reduces_a_base_model_and_its_projector_identically](../../../../../functions/src/llm/provider/llama_cpp_models/mmproj_core_name_reduces_a_base_model_and_its_projector_identically.md)