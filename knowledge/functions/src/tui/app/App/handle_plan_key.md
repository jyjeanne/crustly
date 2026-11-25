---
type: Rust Method
title: handle_plan_key
resource: src/tui/app.rs#L2162-L2260
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/export_plan_to_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_plan_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/reject
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_input_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_plan_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [export_plan_to_markdown](../../../../../functions/src/tui/app/App/export_plan_to_markdown.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)
- [execute_plan_tasks](../../../../../functions/src/tui/app/App/execute_plan_tasks.md)
- [reject](../../../../../functions/src/plan/PlanDocument/reject.md)
- [set_input_text](../../../../../functions/src/tui/app/App/set_input_text.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)