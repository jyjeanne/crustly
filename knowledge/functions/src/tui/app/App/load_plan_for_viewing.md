---
type: Rust Method
title: load_plan_for_viewing
resource: src/tui/app.rs#L1565-L1633
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/plan/PlanService/get_most_recent_plan
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn load_plan_for_viewing(&mut self) -> Result<()>`

# Calls

- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)