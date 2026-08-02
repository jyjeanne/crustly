---
type: Rust Function
title: mock_sse_server
resource: src/llm/provider/qwen.rs#L1968-L2004
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks
  - functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues
---

# Signature

`async fn mock_sse_server(body: String) -> String`

# Called by

- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)