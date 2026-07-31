---
type: Rust Function
title: try_create_azure
resource: src/llm/provider/factory.rs#L178-L204
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn try_create_azure(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [with_endpoint](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)