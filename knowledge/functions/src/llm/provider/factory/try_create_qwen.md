---
type: Rust Function
title: try_create_qwen
resource: src/llm/provider/factory.rs#L422-L456
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_cn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_create_qwen(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [configure_qwen](../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [dashscope_cn](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_cn.md)
- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)