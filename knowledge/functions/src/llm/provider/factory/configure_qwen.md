---
type: Rust Function
title: configure_qwen
resource: src/llm/provider/factory.rs#L380-L435
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/with_tool_parser
  - functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget
  called_by:
  - functions/src/llm/provider/factory/try_create_qwen
  - functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next
  - functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection
  - functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models
---

# Signature

`fn configure_qwen(mut provider: QwenProvider, config: &QwenProviderConfig) -> QwenProvider`

# Calls

- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [with_thinking_budget](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget.md)

# Called by

- [try_create_qwen](../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [configure_qwen_auto_selects_openai_parser_for_coder_next](../../../../../functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next.md)
- [configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection](../../../../../functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection.md)
- [configure_qwen_keeps_hermes_default_for_other_models](../../../../../functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models.md)