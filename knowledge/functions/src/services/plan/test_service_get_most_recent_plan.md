---
type: Rust Function
title: test_service_get_most_recent_plan
resource: src/services/plan.rs#L485-L527
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_service_get_most_recent_plan()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [get_most_recent_plan](../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)