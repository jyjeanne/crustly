---
type: Rust Function
title: merged_system_prompt
resource: src/llm/provider/llama_cpp.rs#L1412-L1420
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/build_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt_tools_only_still_produces_instructions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt_combines_system_and_tools_with_system_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt_with_neither_is_none
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt_system_only_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt_empty_tools_list_behaves_like_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merged_system_prompt(system: Option<&str>, tools: Option<&[Tool]>) -> Option<String>`

# Called by

- [build_prompt](../../../../../functions/src/llm/provider/llama_cpp/build_prompt.md)
- [merged_system_prompt_tools_only_still_produces_instructions](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_tools_only_still_produces_instructions.md)
- [merged_system_prompt_combines_system_and_tools_with_system_first](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_combines_system_and_tools_with_system_first.md)
- [merged_system_prompt_with_neither_is_none](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_with_neither_is_none.md)
- [merged_system_prompt_system_only_is_unchanged](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_system_only_is_unchanged.md)
- [merged_system_prompt_empty_tools_list_behaves_like_none](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_empty_tools_list_behaves_like_none.md)