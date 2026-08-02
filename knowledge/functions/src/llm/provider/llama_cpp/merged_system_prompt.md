---
type: Rust Function
title: merged_system_prompt
resource: src/llm/provider/llama_cpp.rs#L1311-L1319
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/llama_cpp/build_prompt
  - functions/src/llm/provider/llama_cpp/merged_system_prompt_tools_only_still_produces_instructions
  - functions/src/llm/provider/llama_cpp/merged_system_prompt_combines_system_and_tools_with_system_first
---

# Signature

`fn merged_system_prompt(system: Option<&str>, tools: Option<&[Tool]>) -> Option<String>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [build_prompt](../../../../../functions/src/llm/provider/llama_cpp/build_prompt.md)
- [merged_system_prompt_tools_only_still_produces_instructions](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_tools_only_still_produces_instructions.md)
- [merged_system_prompt_combines_system_and_tools_with_system_first](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_combines_system_and_tools_with_system_first.md)