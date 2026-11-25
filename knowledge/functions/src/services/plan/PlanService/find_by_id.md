---
type: Rust Method
title: find_by_id
resource: src/services/plan.rs#L91-L93
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>>`

# Calls

- [find_by_id](../../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)