---
type: Rust Function
title: test_plan_with_complex_task_graph
resource: src/db/repository/plan.rs#L1207-L1269
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn test_plan_with_complex_task_graph()`

# Calls

- [new](../../../../../functions/src/plan/PlanDocument/new.md)
- [add_task](../../../../../functions/src/plan/PlanDocument/add_task.md)