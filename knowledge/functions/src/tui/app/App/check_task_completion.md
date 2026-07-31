---
type: Rust Method
title: check_task_completion
resource: src/tui/app.rs#L1519-L1561
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/save_plan
  called_by:
  - functions/src/tui/app/App/complete_response
---

# Signature

`async fn check_task_completion(&mut self, response_content: &str) -> Result<bool>`

# Calls

- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)