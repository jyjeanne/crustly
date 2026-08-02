---
type: Rust Method
title: with_tools
resource: src/llm/provider/types.rs#L197-L200
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
  - functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
---

# Signature

`pub fn with_tools(mut self, tools: Vec<Tool>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [streamed_tool_call_reaches_caller](../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)