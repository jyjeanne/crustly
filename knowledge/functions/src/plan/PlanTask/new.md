---
type: Rust Method
title: new
resource: src/plan/mod.rs#L567-L586
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/plan_completion_gate_decision_matrix
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(order: usize, title: String, description: String, task_type: TaskType) -> Self`

# Called by

- [plan_completion_gate_decision_matrix](../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)