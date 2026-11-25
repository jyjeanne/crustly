---
type: Rust Function
title: render_memory
resource: src/tui/render.rs#L880-L1030
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/one_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_memory(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [visible_memories](../../../../functions/src/tui/app/visible_memories.md)
- [skip](../../../../functions/src/plan/PlanTask/skip.md)
- [one_line](../../../../functions/src/tui/render/one_line.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)