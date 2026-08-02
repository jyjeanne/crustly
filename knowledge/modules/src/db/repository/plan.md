---
type: Rust Module
title: plan
resource: src/db/repository/plan.rs#L1-L1317
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-db-models-plan-plantask-plantaskstatus
  - external/crate-plan-plandocument-planstatus-taskstatus-tasktype
  - external/anyhow-context-result
  - external/sqlx-sqlitepool
  - external/uuid-uuid
  - external/chrono-datetime
  - external/super
  - external/crate-db-models-session
  - external/crate-db-repository-session-sessionrepository
  - external/crate-db-database
  - external/crate-plan-plantask-tasktype
  - external/chrono-utc
  member_of:
  - packages/crustly
---

# Contains

- [PlanRepository](../../../../classes/src/db/repository/plan/PlanRepository.md)
- [new](../../../../functions/src/db/repository/plan/PlanRepository/new.md)
- [find_by_id](../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)
- [find_tasks_by_plan_id](../../../../functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id.md)
- [create](../../../../functions/src/db/repository/plan/PlanRepository/create.md)
- [update](../../../../functions/src/db/repository/plan/PlanRepository/update.md)
- [delete](../../../../functions/src/db/repository/plan/PlanRepository/delete.md)
- [plan_from_db](../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)
- [task_from_db](../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)
- [plan_to_db](../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)
- [task_to_db](../../../../functions/src/db/repository/plan/PlanRepository/task_to_db.md)
- [parse_plan_status](../../../../functions/src/db/repository/plan/PlanRepository/parse_plan_status.md)
- [format_plan_status](../../../../functions/src/db/repository/plan/PlanRepository/format_plan_status.md)
- [parse_task_type](../../../../functions/src/db/repository/plan/PlanRepository/parse_task_type.md)
- [format_task_type](../../../../functions/src/db/repository/plan/PlanRepository/format_task_type.md)
- [parse_task_status](../../../../functions/src/db/repository/plan/PlanRepository/parse_task_status.md)
- [format_task_status](../../../../functions/src/db/repository/plan/PlanRepository/format_task_status.md)
- [PlanTaskRepository](../../../../classes/src/db/repository/plan/PlanTaskRepository.md)
- [new](../../../../functions/src/db/repository/plan/PlanTaskRepository/new.md)
- [create_task](../../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [update_task_status](../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)
- [get_task](../../../../functions/src/db/repository/plan/PlanTaskRepository/get_task.md)
- [list_tasks_for_plan](../../../../functions/src/db/repository/plan/PlanTaskRepository/list_tasks_for_plan.md)
- [get_incomplete_tasks](../../../../functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks.md)
- [row_to_plan_task](../../../../functions/src/db/repository/plan/row_to_plan_task.md)
- [setup_test_db](../../../../functions/src/db/repository/plan/setup_test_db.md)
- [create_test_plan](../../../../functions/src/db/repository/plan/create_test_plan.md)
- [test_plan_create](../../../../functions/src/db/repository/plan/test_plan_create.md)
- [test_plan_find_by_id](../../../../functions/src/db/repository/plan/test_plan_find_by_id.md)
- [test_plan_find_by_session_id](../../../../functions/src/db/repository/plan/test_plan_find_by_session_id.md)
- [test_plan_update](../../../../functions/src/db/repository/plan/test_plan_update.md)
- [test_plan_delete](../../../../functions/src/db/repository/plan/test_plan_delete.md)
- [test_plan_tasks_cascade_delete](../../../../functions/src/db/repository/plan/test_plan_tasks_cascade_delete.md)
- [test_plan_status_conversion](../../../../functions/src/db/repository/plan/test_plan_status_conversion.md)
- [test_task_type_conversion](../../../../functions/src/db/repository/plan/test_task_type_conversion.md)
- [test_task_status_conversion](../../../../functions/src/db/repository/plan/test_task_status_conversion.md)
- [test_task_dependencies_serialization](../../../../functions/src/db/repository/plan/test_task_dependencies_serialization.md)
- [test_plan_risks_serialization](../../../../functions/src/db/repository/plan/test_plan_risks_serialization.md)
- [test_plan_with_no_tasks](../../../../functions/src/db/repository/plan/test_plan_with_no_tasks.md)
- [test_plan_update_task_status](../../../../functions/src/db/repository/plan/test_plan_update_task_status.md)
- [test_plan_with_complex_task_graph](../../../../functions/src/db/repository/plan/test_plan_with_complex_task_graph.md)
- [test_multiple_sessions_multiple_plans](../../../../functions/src/db/repository/plan/test_multiple_sessions_multiple_plans.md)

# Imports

- `crate::db::models::{Plan, PlanTask, PlanTaskStatus}`
- `crate::plan::{PlanDocument, PlanStatus, TaskStatus, TaskType}`
- `anyhow::{Context, Result}`
- `sqlx::SqlitePool`
- `uuid::Uuid`
- `chrono::DateTime`
- `super::*`
- `crate::db::models::Session`
- `crate::db::repository::session::SessionRepository`
- `crate::db::Database`
- `crate::plan::{PlanTask, TaskType}`
- `chrono::Utc`

# Member of

- [crustly](../../../../packages/crustly.md)