---
type: Rust Function
title: setup_test_env
resource: tests/plan_mode_integration_test.rs#L18-L41
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
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

`async fn setup_test_env() -> (Database, ServiceContext, PlanService, Session, TempDir)`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)

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