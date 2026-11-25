---
type: Rust Function
title: test_plan_state_transitions
resource: src/plan/plan_tests.rs#L180-L198
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/start_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_plan_state_transitions()`

# Calls

- [approve](../../../../functions/src/plan/PlanDocument/approve.md)
- [start_execution](../../../../functions/src/plan/PlanDocument/start_execution.md)
- [complete](../../../../functions/src/plan/PlanDocument/complete.md)