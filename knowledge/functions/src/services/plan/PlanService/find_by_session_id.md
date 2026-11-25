---
type: Rust Method
title: find_by_session_id
resource: src/services/plan.rs#L96-L98
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/get_plan_history
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/get_completed_plans
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/get_active_plans
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/get_statistics
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [find_by_session_id](../../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)

# Called by

- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [get_plan_history](../../../../../functions/src/services/plan/PlanService/get_plan_history.md)
- [get_completed_plans](../../../../../functions/src/services/plan/PlanService/get_completed_plans.md)
- [get_active_plans](../../../../../functions/src/services/plan/PlanService/get_active_plans.md)
- [get_statistics](../../../../../functions/src/services/plan/PlanService/get_statistics.md)