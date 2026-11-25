---
type: Rust Function
title: merge_by_canonical_path
resource: src/llm/provider/llama_cpp_models.rs#L557-L592
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/is_richer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_by_canonical_path(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel>`

# Calls

- [is_richer](../../../../../functions/src/llm/provider/llama_cpp_models/is_richer.md)

# Called by

- [deduplicate_and_merge](../../../../../functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge.md)