---
type: Rust Method
title: with_base_url
resource: src/llm/provider/qwen.rs#L148-L160
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/build_client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_cn
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_base_url(api_key: String, base_url: String) -> Self`

# Calls

- [build_client](../../../../../../functions/src/llm/provider/qwen/QwenProvider/build_client.md)

# Called by

- [dashscope_intl](../../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [dashscope_cn](../../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_cn.md)