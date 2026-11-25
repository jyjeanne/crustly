---
type: Rust Module
title: openai
resource: src/llm/provider/openai.rs#L1-L1616
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-error-providererror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-provider-providerstream
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-types
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-client
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-retry-retry-with-backoff-retryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-streamext-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-types-llmrequest-message
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-types-contentblock-llmrequest-message-role
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-test-support-held-response-server
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
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
- [ok_events](../../../../functions/src/llm/provider/openai/ok_events.md)
- [StreamedToolCall](../../../../classes/src/llm/provider/openai/StreamedToolCall.md)
- [OpenAIStreamState](../../../../classes/src/llm/provider/openai/OpenAIStreamState.md)
- [new](../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [on_bytes](../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)
- [on_line](../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_line.md)
- [finish](../../../../functions/src/llm/provider/openai/OpenAIStreamState/finish.md)
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
- [test_text_sent_with_tool_results_follows_them](../../../../functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them.md)
- [test_calculate_cost](../../../../functions/src/llm/provider/openai/test_calculate_cost.md)
- [test_calculate_cost_current_models_are_not_free](../../../../functions/src/llm/provider/openai/test_calculate_cost_current_models_are_not_free.md)
- [test_calculate_cost_unknown_model_returns_zero](../../../../functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero.md)
- [test_tool_call_index_in_bounds](../../../../functions/src/llm/provider/openai/test_tool_call_index_in_bounds.md)
- [sse](../../../../functions/src/llm/provider/openai/sse.md)
- [text_chunk](../../../../functions/src/llm/provider/openai/text_chunk.md)
- [texts](../../../../functions/src/llm/provider/openai/texts.md)
- [text_is_forwarded_once_its_line_is_complete](../../../../functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete.md)
- [fragmented_tool_call_is_assembled_when_done](../../../../functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done.md)
- [last_line_without_newline_is_parsed_at_the_end](../../../../functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end.md)
- [empty_stream_still_starts_and_stops_the_message](../../../../functions/src/llm/provider/openai/empty_stream_still_starts_and_stops_the_message.md)
- [text_arrives_before_the_response_is_complete](../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)

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
- `crate::llm::provider::types::{ContentBlock, LLMRequest, Message, Role}`
- `crate::llm::provider::test_support::held_response_server`
- `serde_json::json`

# Member of

- [crustly](../../../../packages/crustly.md)