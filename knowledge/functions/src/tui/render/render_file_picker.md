---
type: Rust Function
title: render_file_picker
resource: src/tui/render.rs#L1941-L2058
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_file_picker(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [skip](../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)