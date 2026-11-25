---
type: Rust Method
title: with_default_model
resource: src/llm/provider/openai.rs#L127-L130
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/configure_openai
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_supports_vision_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_default_model(mut self, model: String) -> Self`

# Called by

- [with_default_model](../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/with_default_model.md)
- [configure_openai](../../../../../../functions/src/llm/provider/factory/configure_openai.md)
- [test_supports_vision_detection](../../../../../../functions/src/llm/provider/openai/test_supports_vision_detection.md)