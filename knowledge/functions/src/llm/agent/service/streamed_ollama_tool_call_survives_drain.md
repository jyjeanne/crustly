---
type: Rust Function
title: streamed_ollama_tool_call_survives_drain
resource: src/llm/agent/service.rs#L2309-L2354
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_system
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn streamed_ollama_tool_call_survives_drain()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_default_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_default_model.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_tools](../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [with_max_tokens](../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_streaming](../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)