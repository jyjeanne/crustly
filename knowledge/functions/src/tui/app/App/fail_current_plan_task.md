---
type: Rust Method
title: fail_current_plan_task
resource: src/tui/app.rs#L2294-L2316
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/save_plan
  called_by:
  - functions/src/tui/app/App/handle_event
---

# Signature

`async fn fail_current_plan_task(&mut self, error: &str) -> Result<()>`

# Calls

- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)