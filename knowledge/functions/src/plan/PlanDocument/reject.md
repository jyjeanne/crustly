---
type: Rust Method
title: reject
resource: src/plan/mod.rs#L189-L192
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/plan/plan_tests/test_plan_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn reject(&mut self)`

# Called by

- [test_plan_rejection](../../../../functions/src/plan/plan_tests/test_plan_rejection.md)
- [handle_plan_key](../../../../functions/src/tui/app/App/handle_plan_key.md)