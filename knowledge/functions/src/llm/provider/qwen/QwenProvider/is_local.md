---
type: Rust Method
title: is_local
resource: src/llm/provider/qwen.rs#L226-L228
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/headers
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/llm/provider/qwen/QwenProvider/provider/validate_model
  - functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost
---

# Signature

`fn is_local(&self) -> bool`

# Called by

- [headers](../../../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [validate_model](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/validate_model.md)
- [calculate_cost](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost.md)