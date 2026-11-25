---
type: Rust Function
title: configure_openai
resource: src/llm/provider/factory.rs#L537-L543
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_openai
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn configure_openai(mut provider: OpenAIProvider, config: &ProviderConfig) -> OpenAIProvider`

# Calls

- [with_default_model](../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_default_model.md)

# Called by

- [try_create_openai](../../../../../functions/src/llm/provider/factory/try_create_openai.md)