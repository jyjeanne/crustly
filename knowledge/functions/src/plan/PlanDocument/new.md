---
type: Rust Method
title: new
resource: src/plan/mod.rs#L60-L76
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/plan/create_test_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/test_plan_with_no_tasks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/test_plan_with_complex_task_graph
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/plan_completion_gate_decision_matrix
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/plan_tests/create_test_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/create_test_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/create_multi_task_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(session_id: Uuid, title: String, description: String) -> Self`

# Called by

- [create_test_plan](../../../../functions/src/db/repository/plan/create_test_plan.md)
- [test_plan_with_no_tasks](../../../../functions/src/db/repository/plan/test_plan_with_no_tasks.md)
- [test_plan_with_complex_task_graph](../../../../functions/src/db/repository/plan/test_plan_with_complex_task_graph.md)
- [plan_completion_gate_decision_matrix](../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [create_test_plan](../../../../functions/src/plan/plan_tests/create_test_plan.md)
- [create_test_plan](../../../../functions/src/services/plan/create_test_plan.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [chat_shows_pending_plan_banner_only_while_awaiting_approval](../../../../functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval.md)
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)
- [create_multi_task_plan](../../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)