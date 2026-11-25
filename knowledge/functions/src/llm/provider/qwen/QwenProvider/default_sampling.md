---
type: Rust Method
title: default_sampling
resource: src/llm/provider/qwen.rs#L928-L946
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_sampling( model: &str, thinking_enabled: bool, ) -> (Option<f32>, Option<u32>, Option<f32>)`

# Calls

- [contains](../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)