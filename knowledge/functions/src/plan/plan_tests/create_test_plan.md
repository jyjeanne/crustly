---
type: Rust Function
title: create_test_plan
resource: src/plan/plan_tests.rs#L11-L17
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
---

# Signature

`fn create_test_plan(session_id: Uuid) -> PlanDocument`

# Calls

- [new](../../../../functions/src/plan/PlanDocument/new.md)