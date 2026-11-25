---
type: Rust Function
title: try_create_azure
resource: src/llm/provider/factory.rs#L229-L253
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_create_azure(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [with_endpoint](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint.md)
- [with_default_model](../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_default_model.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)