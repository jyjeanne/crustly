---
type: Rust Module
title: gemini
resource: src/llm/provider/gemini.rs#L1-L1363
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-providererror-result
  - external/super-r-trait-provider-providerstream
  - external/super-types
  - external/async-trait-async-trait
  - external/reqwest-client
  - external/serde-deserialize-serialize
  - external/std-collections-hashmap
  - external/std-time-duration
  - external/super-retry-retry-with-backoff-retryconfig
  - external/futures-streamext-as
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [GeminiProvider](../../../../classes/src/llm/provider/gemini/GeminiProvider.md)
- [new](../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [with_base_url](../../../../functions/src/llm/provider/gemini/GeminiProvider/with_base_url.md)
- [with_default_model](../../../../functions/src/llm/provider/gemini/GeminiProvider/with_default_model.md)
- [headers](../../../../functions/src/llm/provider/gemini/GeminiProvider/headers.md)
- [generate_url](../../../../functions/src/llm/provider/gemini/GeminiProvider/generate_url.md)
- [stream_url](../../../../functions/src/llm/provider/gemini/GeminiProvider/stream_url.md)
- [to_gemini_request](../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [from_gemini_response](../../../../functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response.md)
- [handle_error](../../../../functions/src/llm/provider/gemini/GeminiProvider/handle_error.md)
- [build_gemini_error](../../../../functions/src/llm/provider/gemini/build_gemini_error.md)
- [gemini_role](../../../../functions/src/llm/provider/gemini/gemini_role.md)
- [parse_gemini_sse](../../../../functions/src/llm/provider/gemini/parse_gemini_sse.md)
- [complete](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [supports_streaming](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/calculate_cost.md)
- [GeminiRequest](../../../../classes/src/llm/provider/gemini/GeminiRequest.md)
- [GeminiContent](../../../../classes/src/llm/provider/gemini/GeminiContent.md)
- [GeminiPart](../../../../classes/src/llm/provider/gemini/GeminiPart.md)
- [text](../../../../functions/src/llm/provider/gemini/GeminiPart/text.md)
- [with_inline_data](../../../../functions/src/llm/provider/gemini/GeminiPart/with_inline_data.md)
- [GeminiInlineData](../../../../classes/src/llm/provider/gemini/GeminiInlineData.md)
- [GeminiFunctionCall](../../../../classes/src/llm/provider/gemini/GeminiFunctionCall.md)
- [GeminiFunctionResponse](../../../../classes/src/llm/provider/gemini/GeminiFunctionResponse.md)
- [GeminiTool](../../../../classes/src/llm/provider/gemini/GeminiTool.md)
- [GeminiFunctionDeclaration](../../../../classes/src/llm/provider/gemini/GeminiFunctionDeclaration.md)
- [GeminiToolConfig](../../../../classes/src/llm/provider/gemini/GeminiToolConfig.md)
- [GeminiFunctionCallingConfig](../../../../classes/src/llm/provider/gemini/GeminiFunctionCallingConfig.md)
- [GeminiGenerationConfig](../../../../classes/src/llm/provider/gemini/GeminiGenerationConfig.md)
- [GeminiThinkingConfig](../../../../classes/src/llm/provider/gemini/GeminiThinkingConfig.md)
- [GeminiResponse](../../../../classes/src/llm/provider/gemini/GeminiResponse.md)
- [GeminiCandidate](../../../../classes/src/llm/provider/gemini/GeminiCandidate.md)
- [GeminiUsageMetadata](../../../../classes/src/llm/provider/gemini/GeminiUsageMetadata.md)
- [GeminiErrorResponse](../../../../classes/src/llm/provider/gemini/GeminiErrorResponse.md)
- [GeminiError](../../../../classes/src/llm/provider/gemini/GeminiError.md)
- [test_gemini_provider_creation](../../../../functions/src/llm/provider/gemini/test_gemini_provider_creation.md)
- [test_custom_default_model](../../../../functions/src/llm/provider/gemini/test_custom_default_model.md)
- [test_supported_models_include_gemma](../../../../functions/src/llm/provider/gemini/test_supported_models_include_gemma.md)
- [test_context_window](../../../../functions/src/llm/provider/gemini/test_context_window.md)
- [test_gemma_cost_is_free](../../../../functions/src/llm/provider/gemini/test_gemma_cost_is_free.md)
- [test_calculate_cost_gemini_flash](../../../../functions/src/llm/provider/gemini/test_calculate_cost_gemini_flash.md)
- [test_role_mapping](../../../../functions/src/llm/provider/gemini/test_role_mapping.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_gemini_request_tool_result_uses_function_name](../../../../functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name.md)
- [test_from_gemini_response_maps_tool_use](../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use.md)
- [test_from_gemini_response_maps_thinking](../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking.md)
- [test_thinking_config_forwarded](../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [test_json_mode_sets_response_mime_type](../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [test_inline_image_becomes_inline_data_part](../../../../functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part.md)
- [test_image_url_source_is_skipped_without_panicking](../../../../functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking.md)
- [test_context_window_all_known_models](../../../../functions/src/llm/provider/gemini/test_context_window_all_known_models.md)
- [test_calculate_cost_all_known_models](../../../../functions/src/llm/provider/gemini/test_calculate_cost_all_known_models.md)
- [test_build_gemini_error_rate_limit_with_retry_after](../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_with_retry_after.md)
- [test_build_gemini_error_rate_limit_without_retry_after](../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_without_retry_after.md)
- [test_build_gemini_error_rate_limit_no_body](../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_no_body.md)
- [test_build_gemini_error_api_error_with_body](../../../../functions/src/llm/provider/gemini/test_build_gemini_error_api_error_with_body.md)
- [test_build_gemini_error_no_body_falls_back_to_unknown](../../../../functions/src/llm/provider/gemini/test_build_gemini_error_no_body_falls_back_to_unknown.md)
- [test_parse_gemini_sse_text_response](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_text_response.md)
- [test_parse_gemini_sse_thinking_part](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_thinking_part.md)
- [test_parse_gemini_sse_function_call](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_function_call.md)
- [test_parse_gemini_sse_max_tokens](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_max_tokens.md)
- [test_parse_gemini_sse_skips_malformed_lines](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_skips_malformed_lines.md)
- [test_parse_gemini_sse_ignores_non_data_lines](../../../../functions/src/llm/provider/gemini/test_parse_gemini_sse_ignores_non_data_lines.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::types::*`
- `async_trait::async_trait`
- `reqwest::Client`
- `serde::{Deserialize, Serialize}`
- `std::collections::HashMap`
- `std::time::Duration`
- `super::retry::{retry_with_backoff, RetryConfig}`
- `futures::StreamExt as _`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)