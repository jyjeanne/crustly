---
type: Rust Method
title: check_and_load_plan
resource: src/tui/app.rs#L1903-L2053
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/plan/PlanService/get_most_recent_plan
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/app/App/complete_response
---

# Signature

`async fn check_and_load_plan(&mut self) -> Result<()>`

# Calls

- [get_most_recent_plan](../../../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)