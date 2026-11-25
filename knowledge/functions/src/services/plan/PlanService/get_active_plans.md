---
type: Rust Method
title: get_active_plans
resource: src/services/plan.rs#L256-L267
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_active_plans(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [find_by_session_id](../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)