---
type: Rust Method
title: new
resource: src/llm/provider/azure.rs#L39-L57
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/azure/test_azure_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/test_azure_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/test_azure_cost_calculation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/azure/test_azure_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(api_key: String, resource_name: String, deployment_id: String) -> Self`

# Calls

- [with_base_url](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_base_url.md)
- [with_api_key_header](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header.md)

# Called by

- [test_azure_provider_creation](../../../../../../functions/src/llm/provider/azure/test_azure_provider_creation.md)
- [test_azure_context_window](../../../../../../functions/src/llm/provider/azure/test_azure_context_window.md)
- [test_azure_cost_calculation](../../../../../../functions/src/llm/provider/azure/test_azure_cost_calculation.md)
- [test_azure_supported_models](../../../../../../functions/src/llm/provider/azure/test_azure_supported_models.md)