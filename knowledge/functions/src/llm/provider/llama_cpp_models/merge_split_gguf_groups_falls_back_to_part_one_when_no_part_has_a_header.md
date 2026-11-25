---
type: Rust Function
title: merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header
resource: src/llm/provider/llama_cpp_models.rs#L1873-L1887
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/fixture_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header()`

# Calls

- [fixture_model](../../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)
- [merge_split_gguf_groups](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)