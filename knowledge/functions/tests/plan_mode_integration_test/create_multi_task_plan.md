---
type: Rust Function
title: create_multi_task_plan
resource: tests/plan_mode_integration_test.rs#L44-L124
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/plan_mode_integration_test/test_end_to_end_plan_creation_and_retrieval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_multiple_concurrent_plans_for_same_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_plan_deletion_with_cascade
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_json_export_import_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_plan_rejection_workflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_multi_task_plan(session_id: Uuid) -> PlanDocument`

# Calls

- [new](../../../functions/src/plan/PlanDocument/new.md)
- [add_task](../../../functions/src/plan/PlanDocument/add_task.md)

# Called by

- [test_end_to_end_plan_creation_and_retrieval](../../../functions/tests/plan_mode_integration_test/test_end_to_end_plan_creation_and_retrieval.md)
- [test_plan_state_transition_workflow](../../../functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow.md)
- [test_multiple_concurrent_plans_for_same_session](../../../functions/tests/plan_mode_integration_test/test_multiple_concurrent_plans_for_same_session.md)
- [test_multiple_sessions_with_separate_plans](../../../functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans.md)
- [test_plan_deletion_with_cascade](../../../functions/tests/plan_mode_integration_test/test_plan_deletion_with_cascade.md)
- [test_json_export_import_integration](../../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)
- [test_plan_rejection_workflow](../../../functions/tests/plan_mode_integration_test/test_plan_rejection_workflow.md)
- [test_task_blocking_and_failure_scenarios](../../../functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios.md)
- [test_get_most_recent_plan_integration](../../../functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration.md)