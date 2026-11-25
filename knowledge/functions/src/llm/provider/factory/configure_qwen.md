---
type: Rust Function
title: configure_qwen
resource: src/llm/provider/factory.rs#L459-L508
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/with_tool_parser
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/with_sampling
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn configure_qwen(mut provider: QwenProvider, config: &QwenProviderConfig) -> QwenProvider`

# Calls

- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [with_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking.md)
- [with_thinking_budget](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget.md)
- [with_default_model](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_default_model.md)
- [with_sampling](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_sampling.md)

# Called by

- [try_create_qwen](../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [configure_qwen_auto_selects_openai_parser_for_coder_next](../../../../../functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next.md)
- [configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection](../../../../../functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection.md)
- [configure_qwen_keeps_hermes_default_for_other_models](../../../../../functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models.md)