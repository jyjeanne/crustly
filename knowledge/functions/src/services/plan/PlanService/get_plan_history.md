---
type: Rust Method
title: get_plan_history
resource: src/services/plan.rs#L242-L244
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_plan_history(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [find_by_session_id](../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)