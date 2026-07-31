---
type: Rust Module
title: anthropic
resource: src/llm/provider/anthropic.rs#L1-L620
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-providererror-result
  - external/super-r-trait-provider-providerstream
  - external/super-types
  - external/async-trait-async-trait
  - external/futures-stream-streamext
  - external/reqwest-client
  - external/serde-deserialize-serialize
  - external/std-time-duration
  - external/super-retry-retry-with-backoff-retryconfig
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [AnthropicProvider](../../../../classes/src/llm/provider/anthropic/AnthropicProvider.md)
- [new](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)
- [with_client](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/with_client.md)
- [headers](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/headers.md)
- [to_anthropic_request](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request.md)
- [from_anthropic_response](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/from_anthropic_response.md)
- [handle_error](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/handle_error.md)
- [complete](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [supports_streaming](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/calculate_cost.md)
- [parse_anthropic_sse_stream](../../../../functions/src/llm/provider/anthropic/parse_anthropic_sse_stream.md)
- [AnthropicRequest](../../../../classes/src/llm/provider/anthropic/AnthropicRequest.md)
- [AnthropicResponse](../../../../classes/src/llm/provider/anthropic/AnthropicResponse.md)
- [AnthropicTokenUsage](../../../../classes/src/llm/provider/anthropic/AnthropicTokenUsage.md)
- [AnthropicError](../../../../classes/src/llm/provider/anthropic/AnthropicError.md)
- [AnthropicErrorDetail](../../../../classes/src/llm/provider/anthropic/AnthropicErrorDetail.md)
- [test_anthropic_provider_creation](../../../../functions/src/llm/provider/anthropic/test_anthropic_provider_creation.md)
- [test_supported_models](../../../../functions/src/llm/provider/anthropic/test_supported_models.md)
- [test_context_window](../../../../functions/src/llm/provider/anthropic/test_context_window.md)
- [test_cost_calculation](../../../../functions/src/llm/provider/anthropic/test_cost_calculation.md)
- [test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids](../../../../functions/src/llm/provider/anthropic/test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids.md)
- [test_cost_calculation_unknown_model_family_returns_zero](../../../../functions/src/llm/provider/anthropic/test_cost_calculation_unknown_model_family_returns_zero.md)
- [sse_stream_yields_every_event_in_a_single_chunk](../../../../functions/src/llm/provider/anthropic/sse_stream_yields_every_event_in_a_single_chunk.md)
- [sse_stream_reassembles_an_event_split_across_chunks](../../../../functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks.md)
- [test_capabilities](../../../../functions/src/llm/provider/anthropic/test_capabilities.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::types::*`
- `async_trait::async_trait`
- `futures::stream::StreamExt`
- `reqwest::Client`
- `serde::{Deserialize, Serialize}`
- `std::time::Duration`
- `super::retry::{retry_with_backoff, RetryConfig}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)