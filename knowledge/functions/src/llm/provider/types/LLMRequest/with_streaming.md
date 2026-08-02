---
type: Rust Method
title: with_streaming
resource: src/llm/provider/types.rs#L253-L256
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_streaming
  - functions/src/llm/agent/service/AgentService/call_provider_streaming
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/provider/types/test_llm_request_builder
  - functions/tests/streaming_test/test_streaming_basic
  - functions/tests/streaming_test/test_streaming_single_chunk
  - functions/tests/streaming_test/test_streaming_multiple_chunks
  - functions/tests/streaming_test/test_streaming_token_counting
  - functions/tests/streaming_test/test_streaming_stop_reason
  - functions/tests/streaming_test/test_streaming_error_handling
  - functions/tests/streaming_test/test_streaming_empty_response
  - functions/tests/streaming_test/test_streaming_content_accumulation
  - functions/tests/streaming_test/test_streaming_request_builder
---

# Signature

`pub fn with_streaming(mut self) -> Self`

# Called by

- [send_message_streaming](../../../../../../functions/src/llm/agent/service/AgentService/send_message_streaming.md)
- [call_provider_streaming](../../../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)
- [test_streaming_basic](../../../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../../../../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../../../../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../../../../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_error_handling](../../../../../../functions/tests/streaming_test/test_streaming_error_handling.md)
- [test_streaming_empty_response](../../../../../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../../../../../functions/tests/streaming_test/test_streaming_content_accumulation.md)
- [test_streaming_request_builder](../../../../../../functions/tests/streaming_test/test_streaming_request_builder.md)