---
type: Rust Method
title: with_api_key_header
resource: src/llm/provider/openai.rs#L138-L141
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/azure/AzureOpenAIProvider/new
  - functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint
  - functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer
---

# Signature

`pub(crate) fn with_api_key_header(mut self) -> Self`

# Called by

- [new](../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [with_endpoint](../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint.md)
- [with_api_key_header_sends_api_key_not_bearer](../../../../../../functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer.md)