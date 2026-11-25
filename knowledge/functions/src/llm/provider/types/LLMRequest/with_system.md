---
type: Rust Method
title: with_system
resource: src/llm/provider/types.rs#L191-L194
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/test_llm_request_builder
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_system(mut self, system: impl Into<String>) -> Self`

# Called by

- [build_request](../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)