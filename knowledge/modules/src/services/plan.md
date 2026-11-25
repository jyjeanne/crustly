---
type: Rust Module
title: plan
resource: src/services/plan.rs#L1-L658
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crate-db-models-plantaskstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-planrepository-plantaskrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-plan-plandocument-planstatus-taskstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-servicecontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-models-session
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-session-sessionrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-plan-planstatus-plantask-taskstatus-tasktype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PlanValidationWarning](../../../classes/src/services/plan/PlanValidationWarning.md)
- [WarningSeverity](../../../classes/src/services/plan/WarningSeverity.md)
- [PlanStatistics](../../../classes/src/services/plan/PlanStatistics.md)
- [PlanService](../../../classes/src/services/plan/PlanService.md)
- [new](../../../functions/src/services/plan/PlanService/new.md)
- [begin_task](../../../functions/src/services/plan/PlanService/begin_task.md)
- [complete_task](../../../functions/src/services/plan/PlanService/complete_task.md)
- [fail_task](../../../functions/src/services/plan/PlanService/fail_task.md)
- [get_incomplete_tasks](../../../functions/src/services/plan/PlanService/get_incomplete_tasks.md)
- [find_by_id](../../../functions/src/services/plan/PlanService/find_by_id.md)
- [find_by_session_id](../../../functions/src/services/plan/PlanService/find_by_session_id.md)
- [get_most_recent_plan](../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [create](../../../functions/src/services/plan/PlanService/create.md)
- [update](../../../functions/src/services/plan/PlanService/update.md)
- [delete](../../../functions/src/services/plan/PlanService/delete.md)
- [export_to_json](../../../functions/src/services/plan/PlanService/export_to_json.md)
- [import_from_json](../../../functions/src/services/plan/PlanService/import_from_json.md)
- [validate_plan](../../../functions/src/services/plan/PlanService/validate_plan.md)
- [get_plan_history](../../../functions/src/services/plan/PlanService/get_plan_history.md)
- [get_completed_plans](../../../functions/src/services/plan/PlanService/get_completed_plans.md)
- [get_active_plans](../../../functions/src/services/plan/PlanService/get_active_plans.md)
- [get_statistics](../../../functions/src/services/plan/PlanService/get_statistics.md)
- [setup_test_service](../../../functions/src/services/plan/setup_test_service.md)
- [create_test_plan](../../../functions/src/services/plan/create_test_plan.md)
- [test_service_create_and_find](../../../functions/src/services/plan/test_service_create_and_find.md)
- [test_service_update](../../../functions/src/services/plan/test_service_update.md)
- [test_service_delete](../../../functions/src/services/plan/test_service_delete.md)
- [test_service_find_by_session_id](../../../functions/src/services/plan/test_service_find_by_session_id.md)
- [test_service_get_most_recent_plan](../../../functions/src/services/plan/test_service_get_most_recent_plan.md)
- [test_service_export_to_json](../../../functions/src/services/plan/test_service_export_to_json.md)
- [test_service_import_from_json](../../../functions/src/services/plan/test_service_import_from_json.md)
- [test_service_export_import_roundtrip](../../../functions/src/services/plan/test_service_export_import_roundtrip.md)
- [test_service_atomic_json_write](../../../functions/src/services/plan/test_service_atomic_json_write.md)
- [test_service_json_import_nonexistent_file](../../../functions/src/services/plan/test_service_json_import_nonexistent_file.md)
- [test_service_json_import_invalid_json](../../../functions/src/services/plan/test_service_json_import_invalid_json.md)

# Imports

- `crate::db::models::PlanTaskStatus`
- `crate::db::repository::{PlanRepository, PlanTaskRepository}`
- `crate::plan::{PlanDocument, PlanStatus, TaskStatus}`
- `crate::services::ServiceContext`
- `anyhow::Result`
- `uuid::Uuid`
- `super::*`
- `crate::db::models::Session`
- `crate::db::repository::session::SessionRepository`
- `crate::db::Database`
- `crate::plan::{PlanStatus, PlanTask, TaskStatus, TaskType}`
- `tempfile::TempDir`

# Member of

- [crustly](../../../packages/crustly.md)