---
type: Rust Method
title: new
resource: tests/streaming_test.rs#L23-L74
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/streaming_test/test_streaming_basic
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_single_chunk
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_multiple_chunks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_token_counting
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_stop_reason
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_empty_response
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_content_accumulation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_provider_supports_streaming
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(text_chunks: Vec<&str>) -> Self`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [test_streaming_basic](../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_empty_response](../../../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../../../functions/tests/streaming_test/test_streaming_content_accumulation.md)
- [test_provider_supports_streaming](../../../../functions/tests/streaming_test/test_provider_supports_streaming.md)