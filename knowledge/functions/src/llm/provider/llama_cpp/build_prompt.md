---
type: Rust Function
title: build_prompt
resource: src/llm/provider/llama_cpp.rs#L1426-L1476
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/merged_system_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_prompt( model: &LlamaModel, chat_template: &Option<LlamaChatTemplate>, request: &LLMRequest, ) -> Result<String>`

# Calls

- [merged_system_prompt](../../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt.md)