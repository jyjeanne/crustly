---
type: Rust Function
title: stream_skips_malformed_sse_chunk_and_continues
resource: src/llm/provider/qwen.rs#L2086-L2116
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/mock_sse_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn stream_skips_malformed_sse_chunk_and_continues()`

# Calls

- [mock_sse_server](../../../../../functions/src/llm/provider/qwen/mock_sse_server.md)
- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)