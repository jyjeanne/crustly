---
type: Rust Function
title: setup_test_env
resource: tests/plan_mode_integration_test.rs#L18-L41
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/Session/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
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

`async fn setup_test_env() -> (Database, ServiceContext, PlanService, Session, TempDir)`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/services/plan/PlanService/new.md)
- [new](../../../functions/src/db/repository/session/SessionRepository/new.md)
- [new](../../../functions/src/db/models/Session/new.md)
- [create](../../../functions/src/db/repository/session/SessionRepository/create.md)

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