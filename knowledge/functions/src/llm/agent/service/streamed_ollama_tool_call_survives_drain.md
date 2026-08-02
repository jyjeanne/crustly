---
type: Rust Function
title: streamed_ollama_tool_call_survives_drain
resource: src/llm/agent/service.rs#L2458-L2503
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/types/LLMRequest/with_tools
  - functions/src/llm/provider/types/LLMRequest/with_max_tokens
  - functions/src/llm/provider/types/LLMRequest/with_system
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/llm/agent/service/drain_stream_to_response
---

# Signature

`async fn streamed_ollama_tool_call_survives_drain()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_tools](../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [with_max_tokens](../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_streaming](../../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)