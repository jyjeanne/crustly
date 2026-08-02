---
type: Rust Function
title: build_prompt
resource: src/llm/provider/llama_cpp.rs#L1325-L1375
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/merged_system_prompt
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/llama_cpp/prepare_generation
---

# Signature

`fn build_prompt( model: &LlamaModel, chat_template: &Option<LlamaChatTemplate>, request: &LLMRequest, ) -> Result<String>`

# Calls

- [merged_system_prompt](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)