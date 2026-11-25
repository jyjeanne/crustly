---
type: Rust Method
title: complete
resource: src/llm/provider/azure.rs#L95-L97
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [complete](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)