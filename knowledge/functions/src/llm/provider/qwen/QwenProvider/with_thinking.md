---
type: Rust Method
title: with_thinking
resource: src/llm/provider/qwen.rs#L202-L205
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_mode_configuration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_extraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_thinking(mut self, enabled: bool) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_thinking_mode_configuration](../../../../../../functions/src/llm/provider/qwen/test_thinking_mode_configuration.md)
- [test_thinking_extraction](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction.md)
- [test_thinking_extraction_out_of_order_tags_does_not_panic](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic.md)
- [test_sampling_defaults_qwen3_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)