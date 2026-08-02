---
type: Rust Module
title: streaming_test
resource: tests/streaming_test.rs#L1-L341
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-result
  - external/async-trait-async-trait
  - external/crustly-llm-provider-error-providererror-result-as-providerresult-types-contentblock-contentdelta-llmrequest-llmresponse-messagedelta-role-stopreason-streamevent-streammessage-tokenusage-provider-providerstream
  - external/futures-stream-streamext
  member_of:
  - packages/crustly
---

# Contains

- [StreamingMockProvider](../../classes/tests/streaming_test/StreamingMockProvider.md)
- [new](../../functions/tests/streaming_test/StreamingMockProvider/new.md)
- [with_error](../../functions/tests/streaming_test/StreamingMockProvider/with_error.md)
- [complete](../../functions/tests/streaming_test/StreamingMockProvider/provider/complete.md)
- [stream](../../functions/tests/streaming_test/StreamingMockProvider/provider/stream.md)
- [name](../../functions/tests/streaming_test/StreamingMockProvider/provider/name.md)
- [default_model](../../functions/tests/streaming_test/StreamingMockProvider/provider/default_model.md)
- [supported_models](../../functions/tests/streaming_test/StreamingMockProvider/provider/supported_models.md)
- [context_window](../../functions/tests/streaming_test/StreamingMockProvider/provider/context_window.md)
- [calculate_cost](../../functions/tests/streaming_test/StreamingMockProvider/provider/calculate_cost.md)
- [supports_streaming](../../functions/tests/streaming_test/StreamingMockProvider/provider/supports_streaming.md)
- [test_streaming_basic](../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_error_handling](../../functions/tests/streaming_test/test_streaming_error_handling.md)
- [test_streaming_empty_response](../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../functions/tests/streaming_test/test_streaming_content_accumulation.md)
- [test_streaming_request_builder](../../functions/tests/streaming_test/test_streaming_request_builder.md)
- [test_provider_supports_streaming](../../functions/tests/streaming_test/test_provider_supports_streaming.md)

# Imports

- `anyhow::Result`
- `async_trait::async_trait`
- `crustly::llm::provider::{
    error::{ProviderError, Result as ProviderResult},
    types::{
        ContentBlock, ContentDelta, LLMRequest, LLMResponse, MessageDelta, Role, StopReason,
        StreamEvent, StreamMessage, TokenUsage,
    },
    Provider, ProviderStream,
}`
- `futures::{stream, StreamExt}`

# Member of

- [crustly](../../packages/crustly.md)