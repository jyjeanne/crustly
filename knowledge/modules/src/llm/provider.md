---
type: Rust Module
title: provider
resource: src/llm/provider/mod.rs#L1-L55
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-error-providererror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-r-trait-provider-providercapabilities-providerstream
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-types
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-anthropic-anthropicprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-azure-azureopenaiprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-factory-ollama-provider-from-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-factory-create-named-provider-create-provider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-gemini-geminiprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-llama-cpp-llamacppprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-ollama-ollamaprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-openai-openaiprovider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-qwen-qwenprovider-thinkingconfig-toolcallparser
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use error::{ProviderError, Result}`
- `pub use r#trait::{Provider, ProviderCapabilities, ProviderStream}`
- `pub use types::*`
- `pub use anthropic::AnthropicProvider`
- `pub use azure::AzureOpenAIProvider`
- `pub use factory::ollama_provider_from_config`
- `pub use factory::{create_named_provider, create_provider}`
- `pub use gemini::GeminiProvider`
- `pub use llama_cpp::LlamaCppProvider`
- `pub use ollama::OllamaProvider`
- `pub use openai::OpenAIProvider`
- `pub use qwen::{QwenProvider, ThinkingConfig, ToolCallParser}`

# Member of

- [crustly](../../../packages/crustly.md)