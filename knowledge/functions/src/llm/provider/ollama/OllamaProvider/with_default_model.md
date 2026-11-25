---
type: Rust Method
title: with_default_model
resource: src/llm/provider/ollama.rs#L245-L248
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_supports_vision_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/ollama_download/build_ollama_provider
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_default_model(mut self, model: String) -> Self`

# Called by

- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [test_with_default_model](../../../../../../functions/src/llm/provider/ollama/test_with_default_model.md)
- [test_supports_vision_detection](../../../../../../functions/src/llm/provider/ollama/test_supports_vision_detection.md)
- [streamed_tool_call_reaches_caller](../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [build_ollama_provider](../../../../../../functions/src/tui/ollama_download/build_ollama_provider.md)