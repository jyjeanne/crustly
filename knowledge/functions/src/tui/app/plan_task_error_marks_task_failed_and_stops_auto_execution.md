---
type: Rust Function
title: plan_task_error_marks_task_failed_and_stops_auto_execution
resource: src/tui/app.rs#L5724-L5766
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn plan_task_error_marks_task_failed_and_stops_auto_execution()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [new](../../../../functions/src/plan/PlanDocument/new.md)
- [new](../../../../functions/src/plan/PlanTask/new.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)