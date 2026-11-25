---
type: Rust Function
title: plan_mode_shows_full_document_with_tasks_and_criteria
resource: src/tui/render.rs#L4013-L4044
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn plan_mode_shows_full_document_with_tasks_and_criteria()`

# Calls

- [new](../../../../functions/src/plan/PlanDocument/new.md)
- [new](../../../../functions/src/plan/PlanTask/new.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)