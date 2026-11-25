---
type: Rust Function
title: list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous
resource: src/llm/provider/llama_cpp_models.rs#L1978-L1992
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)