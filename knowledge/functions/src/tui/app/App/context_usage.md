---
type: Rust Method
title: context_usage
resource: src/tui/app.rs#L519-L521
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/render/render_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn context_usage(&self) -> Option<crate::llm::agent::ContextUsage>`

# Called by

- [render_header](../../../../../functions/src/tui/render/render_header.md)
- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)