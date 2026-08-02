---
type: Rust Module
title: azure
resource: src/llm/provider/azure.rs#L1-L196
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-openai-openaiprovider-llmrequest-llmresponse-provider-result
  - external/async-trait-async-trait
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [AzureOpenAIProvider](../../../../classes/src/llm/provider/azure/AzureOpenAIProvider.md)
- [new](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/new.md)
- [with_endpoint](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_endpoint.md)
- [with_default_model](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_default_model.md)
- [name](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/default_model.md)
- [complete](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/stream.md)
- [supported_models](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/calculate_cost.md)
- [test_azure_provider_creation](../../../../functions/src/llm/provider/azure/test_azure_provider_creation.md)
- [test_azure_context_window](../../../../functions/src/llm/provider/azure/test_azure_context_window.md)
- [test_azure_cost_calculation](../../../../functions/src/llm/provider/azure/test_azure_cost_calculation.md)
- [test_azure_supported_models](../../../../functions/src/llm/provider/azure/test_azure_supported_models.md)

# Imports

- `super::{openai::OpenAIProvider, LLMRequest, LLMResponse, Provider, Result}`
- `async_trait::async_trait`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)