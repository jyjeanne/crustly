---
type: Rust Method
title: block
resource: src/plan/mod.rs#L692-L694
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/plan/plan_tests/test_task_blocking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_help
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn block(&mut self, reason: String)`

# Called by

- [test_task_blocking](../../../../functions/src/plan/plan_tests/test_task_blocking.md)
- [render_chat](../../../../functions/src/tui/render/render_chat.md)
- [render_help](../../../../functions/src/tui/render/render_help.md)