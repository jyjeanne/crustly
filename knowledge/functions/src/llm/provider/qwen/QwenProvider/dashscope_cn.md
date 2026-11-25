---
type: Rust Method
title: dashscope_cn
resource: src/llm/provider/qwen.rs#L128-L130
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_qwen
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn dashscope_cn(api_key: String) -> Self`

# Calls

- [with_base_url](../../../../../../functions/src/llm/provider/qwen/QwenProvider/with_base_url.md)

# Called by

- [try_create_qwen](../../../../../../functions/src/llm/provider/factory/try_create_qwen.md)