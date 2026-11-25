---
type: Rust Method
title: find_tasks_by_plan_id
resource: src/db/repository/plan.rs#L69-L79
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
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

`async fn find_tasks_by_plan_id(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [find_by_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)