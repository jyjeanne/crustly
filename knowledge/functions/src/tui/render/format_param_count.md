---
type: Rust Function
title: format_param_count
resource: src/tui/render.rs#L2556-L2566
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/llama_cpp_model_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/format_param_count_formats_at_the_expected_scale
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_param_count(n: u64) -> String`

# Called by

- [llama_cpp_model_lines](../../../../functions/src/tui/render/llama_cpp_model_lines.md)
- [format_param_count_formats_at_the_expected_scale](../../../../functions/src/tui/render/format_param_count_formats_at_the_expected_scale.md)