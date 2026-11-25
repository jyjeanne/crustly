---
type: Rust Method
title: fail_current_plan_task
resource: src/tui/app.rs#L3138-L3160
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fail_current_plan_task(&mut self, error: &str) -> Result<()>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)