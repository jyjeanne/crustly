---
type: Rust Function
title: syntect_to_ratatui_color
resource: src/tui/highlight.rs#L29-L31
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/highlight/syntect_style_to_ratatui
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color`

# Called by

- [syntect_style_to_ratatui](../../../../functions/src/tui/highlight/syntect_style_to_ratatui.md)