---
type: Rust Method
title: validate_model
resource: src/llm/provider/qwen.rs#L1572-L1578
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/is_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn validate_model(&self, model: &str) -> bool`

# Calls

- [is_local](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)
- [supported_models](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supported_models.md)