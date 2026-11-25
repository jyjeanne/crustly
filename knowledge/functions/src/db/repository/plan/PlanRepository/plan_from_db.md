---
type: Rust Method
title: plan_from_db
resource: src/db/repository/plan.rs#L261-L289
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/parse_plan_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/task_from_db
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/plan/PlanRepository/find_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/find_by_session_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plan_from_db(&self, db_plan: Plan, db_tasks: Vec<PlanTask>) -> Result<PlanDocument>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [parse_plan_status](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_plan_status.md)
- [task_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)

# Called by

- [find_by_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)