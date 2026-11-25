---
type: Rust Module
title: plan_mode_integration_test
resource: tests/plan_mode_integration_test.rs#L1-L467
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crustly-db-models-session
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-session-sessionrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-plan-plandocument-planstatus-plantask-taskstatus-tasktype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-services-planservice-servicecontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [setup_test_env](../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [test_end_to_end_plan_creation_and_retrieval](../../functions/tests/plan_mode_integration_test/test_end_to_end_plan_creation_and_retrieval.md)
- [test_plan_state_transition_workflow](../../functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow.md)
- [test_multiple_concurrent_plans_for_same_session](../../functions/tests/plan_mode_integration_test/test_multiple_concurrent_plans_for_same_session.md)
- [test_multiple_sessions_with_separate_plans](../../functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans.md)
- [test_plan_deletion_with_cascade](../../functions/tests/plan_mode_integration_test/test_plan_deletion_with_cascade.md)
- [test_json_export_import_integration](../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)
- [test_plan_rejection_workflow](../../functions/tests/plan_mode_integration_test/test_plan_rejection_workflow.md)
- [test_task_blocking_and_failure_scenarios](../../functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios.md)
- [test_get_most_recent_plan_integration](../../functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration.md)

# Imports

- `crustly::db::models::Session`
- `crustly::db::repository::session::SessionRepository`
- `crustly::db::Database`
- `crustly::plan::{PlanDocument, PlanStatus, PlanTask, TaskStatus, TaskType}`
- `crustly::services::{PlanService, ServiceContext}`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)