---
type: Rust Method
title: with_system
resource: src/llm/provider/types.rs#L191-L194
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/types/test_llm_request_builder
---

# Signature

`pub fn with_system(mut self, system: impl Into<String>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)