---
type: Rust Function
title: create_test_plan
resource: src/db/repository/plan.rs#L743-L796
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_test_plan(session_id: Uuid) -> PlanDocument`

# Calls

- [new](../../../../../functions/src/plan/PlanDocument/new.md)
- [add_task](../../../../../functions/src/plan/PlanDocument/add_task.md)