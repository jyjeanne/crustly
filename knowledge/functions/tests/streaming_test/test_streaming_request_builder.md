---
type: Rust Function
title: test_streaming_request_builder
resource: tests/streaming_test.rs#L330-L335
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_streaming_request_builder()`

# Calls

- [new](../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)