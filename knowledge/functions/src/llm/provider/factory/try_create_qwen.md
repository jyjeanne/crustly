---
type: Rust Function
title: try_create_qwen
resource: src/llm/provider/factory.rs#L386-L423
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/factory/configure_qwen
  - functions/src/llm/provider/qwen/QwenProvider/dashscope_cn
  - functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
  called_by:
  - functions/src/llm/provider/factory/create_provider
---

# Signature

`fn try_create_qwen(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [configure_qwen](../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [dashscope_cn](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_cn.md)
- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)

# Called by

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)