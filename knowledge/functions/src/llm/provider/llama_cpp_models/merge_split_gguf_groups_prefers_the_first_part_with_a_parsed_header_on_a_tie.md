---
type: Rust Function
title: merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie
resource: src/llm/provider/llama_cpp_models.rs#L1849-L1870
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie()`

# Calls

- [fixture_model](../../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)
- [merge_split_gguf_groups](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)