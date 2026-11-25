---
type: Rust Function
title: input_box_height
resource: src/tui/render.rs#L95-L97
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/the_input_box_grows_with_its_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn input_box_height(app: &App) -> u16`

# Called by

- [render](../../../../functions/src/tui/render/render.md)
- [the_input_box_grows_with_its_text](../../../../functions/src/tui/render/the_input_box_grows_with_its_text.md)