---
type: Rust Method
title: handle_plan_key
resource: src/tui/app.rs#L1070-L1168
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/is_cancel
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/app/App/export_plan_to_markdown
  - functions/src/tui/app/App/save_plan
  - functions/src/tui/app/App/execute_plan_tasks
  - functions/src/plan/PlanDocument/reject
  - functions/src/tui/app/App/set_input_text
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn handle_plan_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [export_plan_to_markdown](../../../../../functions/src/tui/app/App/export_plan_to_markdown.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)
- [execute_plan_tasks](../../../../../functions/src/tui/app/App/execute_plan_tasks.md)
- [reject](../../../../../functions/src/plan/PlanDocument/reject.md)
- [set_input_text](../../../../../functions/src/tui/app/App/set_input_text.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)