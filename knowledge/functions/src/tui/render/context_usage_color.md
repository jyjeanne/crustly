---
type: Rust Function
title: context_usage_color
resource: src/tui/render.rs#L190-L199
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextUsage/overhead_is_tight
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn context_usage_color(usage: &crate::llm::agent::ContextUsage) -> Color`

# Calls

- [overhead_is_tight](../../../../functions/src/llm/agent/budget/ContextUsage/overhead_is_tight.md)

# Called by

- [render_header](../../../../functions/src/tui/render/render_header.md)
- [render_model_info](../../../../functions/src/tui/render/render_model_info.md)