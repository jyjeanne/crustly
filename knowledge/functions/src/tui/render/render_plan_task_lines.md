---
type: Rust Function
title: render_plan_task_lines
resource: src/tui/render.rs#L1012-L1047
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/render/render_plan_document
---

# Signature

`fn render_plan_task_lines(task: &crate::plan::PlanTask, idx: usize) -> Vec<Line<'_>>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [render_plan_document](../../../../functions/src/tui/render/render_plan_document.md)