---
type: Rust Function
title: syntect_style_to_ratatui
resource: src/tui/highlight.rs#L34-L48
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/highlight/syntect_to_ratatui_color
  called_by:
  - functions/src/tui/highlight/highlight_code
---

# Signature

`fn syntect_style_to_ratatui(syntect_style: syntect::highlighting::Style) -> Style`

# Calls

- [syntect_to_ratatui_color](../../../../functions/src/tui/highlight/syntect_to_ratatui_color.md)

# Called by

- [highlight_code](../../../../functions/src/tui/highlight/highlight_code.md)