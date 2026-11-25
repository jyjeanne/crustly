---
type: Rust Method
title: stream
resource: src/llm/provider/azure.rs#L99-L101
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<super::ProviderStream>`

# Calls

- [stream](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)