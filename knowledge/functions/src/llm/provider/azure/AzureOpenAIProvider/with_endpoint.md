---
type: Rust Method
title: with_endpoint
resource: src/llm/provider/azure.rs#L68-L76
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header
  called_by:
  - functions/src/llm/provider/factory/try_create_azure
---

# Signature

`pub fn with_endpoint(api_key: String, endpoint: String) -> Self`

# Calls

- [with_api_key_header](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header.md)

# Called by

- [try_create_azure](../../../../../../functions/src/llm/provider/factory/try_create_azure.md)