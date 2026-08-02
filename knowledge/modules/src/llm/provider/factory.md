---
type: Rust Module
title: factory
resource: src/llm/provider/factory.rs#L1-L1166
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-anthropic-anthropicprovider-azure-azureopenaiprovider-error-providererror-gemini-geminiprovider-openai-openaiprovider-qwen-qwenprovider-toolcallparser-provider
  - external/crate-config-config-providerconfig-qwenproviderconfig
  - external/anyhow-context-result
  - external/async-trait-async-trait
  - external/std-sync-arc
  - external/super-ollama-modeloverrides-ollamaprovider
  - external/super-super-error-providererror-result-as-providerresult-r-trait-providerstream-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage
  - external/super
  - external/crate-config-config-providerconfig-providerconfigs-qwenproviderconfig
  - external/std-sync-atomic-atomicusize-ordering
  member_of:
  - packages/crustly
---

# Contains

- [FailoverProvider](../../../../classes/src/llm/provider/factory/FailoverProvider.md)
- [new](../../../../functions/src/llm/provider/factory/FailoverProvider/new.md)
- [is_failover_error](../../../../functions/src/llm/provider/factory/FailoverProvider/is_failover_error.md)
- [complete](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/stream.md)
- [name](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/factory/FailoverProvider/provider/calculate_cost.md)
- [create_provider](../../../../functions/src/llm/provider/factory/create_provider.md)
- [try_create_azure](../../../../functions/src/llm/provider/factory/try_create_azure.md)
- [try_create_gemini](../../../../functions/src/llm/provider/factory/try_create_gemini.md)
- [try_create_ollama](../../../../functions/src/llm/provider/factory/try_create_ollama.md)
- [ollama_provider_from_config](../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [try_create_ollama](../../../../functions/src/llm/provider/factory/try_create_ollama-2.md)
- [try_create_llama_cpp](../../../../functions/src/llm/provider/factory/try_create_llama_cpp.md)
- [try_create_llama_cpp](../../../../functions/src/llm/provider/factory/try_create_llama_cpp-2.md)
- [try_create_qwen](../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [configure_qwen](../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [try_create_openai](../../../../functions/src/llm/provider/factory/try_create_openai.md)
- [configure_openai](../../../../functions/src/llm/provider/factory/configure_openai.md)
- [create_anthropic](../../../../functions/src/llm/provider/factory/create_anthropic.md)
- [RateLimitedProvider](../../../../classes/src/llm/provider/factory/RateLimitedProvider.md)
- [complete](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/stream.md)
- [name](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/factory/RateLimitedProvider/provider/calculate_cost.md)
- [SucceedingProvider](../../../../classes/src/llm/provider/factory/SucceedingProvider.md)
- [complete](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/stream.md)
- [name](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/factory/SucceedingProvider/provider/calculate_cost.md)
- [test_failover_on_rate_limit_tries_next_provider](../../../../functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider.md)
- [test_failover_all_fail_returns_last_error](../../../../functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error.md)
- [test_create_provider_with_anthropic](../../../../functions/src/llm/provider/factory/test_create_provider_with_anthropic.md)
- [test_create_provider_with_openai](../../../../functions/src/llm/provider/factory/test_create_provider_with_openai.md)
- [test_create_provider_with_azure](../../../../functions/src/llm/provider/factory/test_create_provider_with_azure.md)
- [test_disabled_azure_falls_through_to_anthropic](../../../../functions/src/llm/provider/factory/test_disabled_azure_falls_through_to_anthropic.md)
- [test_create_provider_with_gemini](../../../../functions/src/llm/provider/factory/test_create_provider_with_gemini.md)
- [test_create_provider_with_gemini_custom_base_url_and_model](../../../../functions/src/llm/provider/factory/test_create_provider_with_gemini_custom_base_url_and_model.md)
- [gemini_without_api_key_falls_through_to_anthropic](../../../../functions/src/llm/provider/factory/gemini_without_api_key_falls_through_to_anthropic.md)
- [disabled_gemini_is_skipped_in_favour_of_the_next_provider](../../../../functions/src/llm/provider/factory/disabled_gemini_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_openai_is_skipped_in_favour_of_the_next_provider](../../../../functions/src/llm/provider/factory/disabled_openai_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_qwen_is_skipped_in_favour_of_the_next_provider](../../../../functions/src/llm/provider/factory/disabled_qwen_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_llama_cpp_is_skipped_in_favour_of_the_next_provider](../../../../functions/src/llm/provider/factory/disabled_llama_cpp_is_skipped_in_favour_of_the_next_provider.md)
- [absent_llama_cpp_config_does_not_affect_resolution](../../../../functions/src/llm/provider/factory/absent_llama_cpp_config_does_not_affect_resolution.md)
- [disabled_anthropic_fallback_fails_with_a_clear_message](../../../../functions/src/llm/provider/factory/disabled_anthropic_fallback_fails_with_a_clear_message.md)
- [test_create_provider_with_qwen](../../../../functions/src/llm/provider/factory/test_create_provider_with_qwen.md)
- [configure_qwen_auto_selects_openai_parser_for_coder_next](../../../../functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next.md)
- [configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection](../../../../functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection.md)
- [configure_qwen_keeps_hermes_default_for_other_models](../../../../functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models.md)
- [test_create_provider_no_credentials](../../../../functions/src/llm/provider/factory/test_create_provider_no_credentials.md)

# Imports

- `super::{
    anthropic::AnthropicProvider,
    azure::AzureOpenAIProvider,
    error::ProviderError,
    gemini::GeminiProvider,
    openai::OpenAIProvider,
    qwen::{QwenProvider, ToolCallParser},
    Provider,
}`
- `crate::config::{Config, ProviderConfig, QwenProviderConfig}`
- `anyhow::{Context, Result}`
- `async_trait::async_trait`
- `std::sync::Arc`
- `super::ollama::{ModelOverrides, OllamaProvider}`
- `super::super::{
        error::{ProviderError, Result as ProviderResult},
        r#trait::ProviderStream,
        types::{ContentBlock, LLMRequest, LLMResponse, StopReason, TokenUsage},
    }`
- `super::*`
- `crate::config::{Config, ProviderConfig, ProviderConfigs, QwenProviderConfig}`
- `std::sync::atomic::{AtomicUsize, Ordering}`

# Member of

- [crustly](../../../../packages/crustly.md)