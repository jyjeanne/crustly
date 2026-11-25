---
type: Rust Method
title: tool_parser
resource: src/llm/provider/qwen.rs#L177-L179
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
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

`pub(crate) fn tool_parser(&self) -> ToolCallParser`

# Called by

- [configure_qwen_auto_selects_openai_parser_for_coder_next](../../../../../../functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next.md)
- [configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection](../../../../../../functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection.md)
- [configure_qwen_keeps_hermes_default_for_other_models](../../../../../../functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models.md)