---
type: Rust Function
title: deduplicate_and_merge
resource: src/llm/provider/llama_cpp_models.rs#L544-L547
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/merge_by_canonical_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn deduplicate_and_merge(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel>`

# Calls

- [merge_by_canonical_path](../../../../../functions/src/llm/provider/llama_cpp_models/merge_by_canonical_path.md)
- [merge_split_gguf_groups](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)

# Called by

- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)