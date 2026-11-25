---
type: Rust Method
title: is_local
resource: src/llm/provider/qwen.rs#L226-L228
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/validate_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_local(&self) -> bool`

# Called by

- [headers](../../../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [validate_model](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/validate_model.md)
- [context_window](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/context_window.md)
- [calculate_cost](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost.md)