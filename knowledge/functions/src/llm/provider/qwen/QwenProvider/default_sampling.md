---
type: Rust Method
title: default_sampling
resource: src/llm/provider/qwen.rs#L918-L936
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
---

# Signature

`fn default_sampling( model: &str, thinking_enabled: bool, ) -> (Option<f32>, Option<u32>, Option<f32>)`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)