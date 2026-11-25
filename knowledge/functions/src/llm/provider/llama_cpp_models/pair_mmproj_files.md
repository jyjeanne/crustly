---
type: Rust Function
title: pair_mmproj_files
resource: src/llm/provider/llama_cpp_models.rs#L700-L750
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/mmproj_core_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pair_mmproj_files(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [mmproj_core_name](../../../../../functions/src/llm/provider/llama_cpp_models/mmproj_core_name.md)

# Called by

- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)