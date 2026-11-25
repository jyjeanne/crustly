---
type: Rust Method
title: get_statistics
resource: src/services/plan.rs#L270-L326
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/progress_percentage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_statistics(&self, session_id: Uuid) -> Result<PlanStatistics>`

# Calls

- [find_by_session_id](../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)
- [progress_percentage](../../../../../functions/src/plan/PlanDocument/progress_percentage.md)