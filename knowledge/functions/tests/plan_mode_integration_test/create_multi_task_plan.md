---
type: Rust Function
title: create_multi_task_plan
resource: tests/plan_mode_integration_test.rs#L44-L124
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/PlanDocument/add_task
  called_by:
  - functions/tests/plan_mode_integration_test/test_end_to_end_plan_creation_and_retrieval
  - functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow
  - functions/tests/plan_mode_integration_test/test_multiple_concurrent_plans_for_same_session
  - functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans
  - functions/tests/plan_mode_integration_test/test_plan_deletion_with_cascade
  - functions/tests/plan_mode_integration_test/test_json_export_import_integration
  - functions/tests/plan_mode_integration_test/test_plan_rejection_workflow
  - functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios
  - functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration
---

# Signature

`fn create_multi_task_plan(session_id: Uuid) -> PlanDocument`

# Calls

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