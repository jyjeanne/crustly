---
type: Rust Function
title: fit_scroll
resource: src/tui/render.rs#L577-L594
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fit_scroll(lines: Vec<Line<'static>>, mut top: usize, width: u16) -> (Vec<Line<'static>>, u16)`

# Calls

- [skip](../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)