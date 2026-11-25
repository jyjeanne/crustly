---
type: Rust Function
title: merge_split_gguf_groups
resource: src/llm/provider/llama_cpp_models.rs#L594-L669
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_split_gguf_groups(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [deduplicate_and_merge](../../../../../functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge.md)
- [merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie.md)
- [merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header.md)