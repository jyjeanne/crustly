---
type: Rust Method
title: get_completed_plans
resource: src/services/plan.rs#L247-L253
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_completed_plans(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [find_by_session_id](../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)