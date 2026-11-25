---
type: Rust Method
title: find_by_session_id
resource: src/db/repository/plan.rs#L50-L66
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/plan_from_db
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [find_tasks_by_plan_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id.md)
- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)

# Called by

- [find_by_session_id](../../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)