---
type: Rust Function
title: provider_icon
resource: src/tui/render.rs#L166-L176
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn provider_icon(provider_name: &str) -> &'static str`

# Called by

- [render_header](../../../../functions/src/tui/render/render_header.md)
- [render_model_info](../../../../functions/src/tui/render/render_model_info.md)