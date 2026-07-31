---
type: Rust Function
title: configure_openai
resource: src/llm/provider/factory.rs#L466-L473
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/try_create_openai
---

# Signature

`fn configure_openai(mut provider: OpenAIProvider, config: &ProviderConfig) -> OpenAIProvider`

# Called by

- [try_create_openai](../../../../../functions/src/llm/provider/factory/try_create_openai.md)