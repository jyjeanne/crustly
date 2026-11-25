---
type: Rust Function
title: syntect_style_to_ratatui
resource: src/tui/highlight.rs#L34-L48
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/highlight/syntect_to_ratatui_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/highlight/highlight_code
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn syntect_style_to_ratatui(syntect_style: syntect::highlighting::Style) -> Style`

# Calls

- [syntect_to_ratatui_color](../../../../functions/src/tui/highlight/syntect_to_ratatui_color.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [highlight_code](../../../../functions/src/tui/highlight/highlight_code.md)