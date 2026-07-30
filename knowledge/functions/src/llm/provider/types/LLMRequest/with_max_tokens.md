---
type: Rust Method
title: with_max_tokens
resource: src/llm/provider/types.rs#L247-L250
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/types/test_llm_request_builder
---

# Signature

`pub fn with_max_tokens(mut self, max_tokens: u32) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)