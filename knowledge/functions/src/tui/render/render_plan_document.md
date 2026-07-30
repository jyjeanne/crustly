---
type: Rust Function
title: render_plan_document
resource: src/tui/render.rs#L1042-L1132
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/render/render_plan_task_lines
  called_by:
  - functions/src/tui/render/render_plan
---

# Signature

`fn render_plan_document(plan: &crate::plan::PlanDocument, area_width: usize) -> Vec<Line<'_>>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [render_plan_task_lines](../../../../functions/src/tui/render/render_plan_task_lines.md)

# Called by

- [render_plan](../../../../functions/src/tui/render/render_plan.md)