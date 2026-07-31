---
type: Rust Module
title: openai
resource: src/llm/provider/openai.rs#L1-L1312
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-providererror-result
  - external/super-r-trait-provider-providerstream
  - external/super-types
  - external/async-trait-async-trait
  - external/reqwest-client
  - external/serde-deserialize-serialize
  - external/std-time-duration
  - external/super-retry-retry-with-backoff-retryconfig
  - external/futures-streamext-as
  - external/super
  - external/crate-llm-provider-types-llmrequest-message
  member_of:
  - packages/crustly
---

# Contains

- [tool_call_index_in_bounds](../../../../functions/src/llm/provider/openai/tool_call_index_in_bounds.md)
- [AuthStyle](../../../../classes/src/llm/provider/openai/AuthStyle.md)
- [OpenAIProvider](../../../../classes/src/llm/provider/openai/OpenAIProvider.md)
- [new](../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [local](../../../../functions/src/llm/provider/openai/OpenAIProvider/local.md)
- [with_base_url](../../../../functions/src/llm/provider/openai/OpenAIProvider/with_base_url.md)
- [with_default_model](../../../../functions/src/llm/provider/openai/OpenAIProvider/with_default_model.md)
- [with_api_key_header](../../../../functions/src/llm/provider/openai/OpenAIProvider/with_api_key_header.md)
- [headers](../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)
- [to_openai_request](../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [from_openai_response](../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [handle_error](../../../../functions/src/llm/provider/openai/OpenAIProvider/handle_error.md)
- [complete](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [ToolCallBuilder](../../../../classes/src/llm/provider/openai/ToolCallBuilder.md)
- [supports_streaming](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/calculate_cost.md)
- [OpenAIRequest](../../../../classes/src/llm/provider/openai/OpenAIRequest.md)
- [OpenAIStreamOptions](../../../../classes/src/llm/provider/openai/OpenAIStreamOptions.md)
- [OpenAIMessage](../../../../classes/src/llm/provider/openai/OpenAIMessage.md)
- [OpenAIToolCall](../../../../classes/src/llm/provider/openai/OpenAIToolCall.md)
- [OpenAIFunctionCall](../../../../classes/src/llm/provider/openai/OpenAIFunctionCall.md)
- [OpenAITool](../../../../classes/src/llm/provider/openai/OpenAITool.md)
- [OpenAIFunction](../../../../classes/src/llm/provider/openai/OpenAIFunction.md)
- [OpenAIResponse](../../../../classes/src/llm/provider/openai/OpenAIResponse.md)
- [OpenAIChoice](../../../../classes/src/llm/provider/openai/OpenAIChoice.md)
- [OpenAIUsage](../../../../classes/src/llm/provider/openai/OpenAIUsage.md)
- [OpenAIStreamChunk](../../../../classes/src/llm/provider/openai/OpenAIStreamChunk.md)
- [OpenAIStreamChoice](../../../../classes/src/llm/provider/openai/OpenAIStreamChoice.md)
- [OpenAIMessageDelta](../../../../classes/src/llm/provider/openai/OpenAIMessageDelta.md)
- [OpenAIToolCallDelta](../../../../classes/src/llm/provider/openai/OpenAIToolCallDelta.md)
- [OpenAIFunctionDelta](../../../../classes/src/llm/provider/openai/OpenAIFunctionDelta.md)
- [OpenAIErrorResponse](../../../../classes/src/llm/provider/openai/OpenAIErrorResponse.md)
- [OpenAIError](../../../../classes/src/llm/provider/openai/OpenAIError.md)
- [test_openai_provider_creation](../../../../functions/src/llm/provider/openai/test_openai_provider_creation.md)
- [test_local_provider_creation](../../../../functions/src/llm/provider/openai/test_local_provider_creation.md)
- [with_api_key_header_sends_api_key_not_bearer](../../../../functions/src/llm/provider/openai/with_api_key_header_sends_api_key_not_bearer.md)
- [default_auth_style_still_sends_bearer](../../../../functions/src/llm/provider/openai/default_auth_style_still_sends_bearer.md)
- [test_supported_models](../../../../functions/src/llm/provider/openai/test_supported_models.md)
- [test_context_window](../../../../functions/src/llm/provider/openai/test_context_window.md)
- [test_supports_vision_detection](../../../../functions/src/llm/provider/openai/test_supports_vision_detection.md)
- [test_llm_request_new_fields](../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [test_calculate_cost](../../../../functions/src/llm/provider/openai/test_calculate_cost.md)
- [test_calculate_cost_unknown_model_returns_zero](../../../../functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero.md)
- [test_tool_call_index_in_bounds](../../../../functions/src/llm/provider/openai/test_tool_call_index_in_bounds.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::types::*`
- `async_trait::async_trait`
- `reqwest::Client`
- `serde::{Deserialize, Serialize}`
- `std::time::Duration`
- `super::retry::{retry_with_backoff, RetryConfig}`
- `futures::StreamExt as _`
- `super::*`
- `crate::llm::provider::types::{LLMRequest, Message}`

# Member of

- [crustly](../../../../packages/crustly.md)