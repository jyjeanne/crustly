---
type: Rust Method
title: with_sampling
resource: src/llm/provider/qwen.rs#L187-L199
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_sampling( mut self, top_p: Option<f32>, top_k: Option<u32>, repetition_penalty: Option<f32>, ) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_sampling_config_override_wins_over_defaults](../../../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)