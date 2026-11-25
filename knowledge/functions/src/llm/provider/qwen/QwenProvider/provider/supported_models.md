---
type: Rust Method
title: supported_models
resource: src/llm/provider/qwen.rs#L1546-L1570
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/validate_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supported_models(&self) -> Vec<String>`

# Called by

- [validate_model](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/validate_model.md)
- [test_supported_models](../../../../../../../functions/src/llm/provider/qwen/test_supported_models.md)