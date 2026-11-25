---
type: Rust Method
title: with_max_tokens
resource: src/llm/provider/types.rs#L247-L250
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
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

`pub fn with_max_tokens(mut self, max_tokens: u32) -> Self`

# Called by

- [build_request](../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)