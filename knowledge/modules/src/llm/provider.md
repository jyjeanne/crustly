---
type: Rust Module
title: provider
resource: src/llm/provider/mod.rs#L1-L48
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/pub-use-error-providererror-result
  - external/pub-use-r-trait-provider-providercapabilities-providerstream
  - external/pub-use-types
  - external/pub-use-anthropic-anthropicprovider
  - external/pub-use-azure-azureopenaiprovider
  - external/pub-use-factory-create-provider
  - external/pub-use-factory-ollama-provider-from-config
  - external/pub-use-gemini-geminiprovider
  - external/pub-use-llama-cpp-llamacppprovider
  - external/pub-use-ollama-ollamaprovider
  - external/pub-use-openai-openaiprovider
  - external/pub-use-qwen-qwenprovider-thinkingconfig-toolcallparser
  member_of:
  - packages/crustly
---

# Imports

- `pub use error::{ProviderError, Result}`
- `pub use r#trait::{Provider, ProviderCapabilities, ProviderStream}`
- `pub use types::*`
- `pub use anthropic::AnthropicProvider`
- `pub use azure::AzureOpenAIProvider`
- `pub use factory::create_provider`
- `pub use factory::ollama_provider_from_config`
- `pub use gemini::GeminiProvider`
- `pub use llama_cpp::LlamaCppProvider`
- `pub use ollama::OllamaProvider`
- `pub use openai::OpenAIProvider`
- `pub use qwen::{QwenProvider, ThinkingConfig, ToolCallParser}`

# Member of

- [crustly](../../../packages/crustly.md)