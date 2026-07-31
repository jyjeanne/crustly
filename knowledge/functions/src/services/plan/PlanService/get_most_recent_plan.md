---
type: Rust Method
title: get_most_recent_plan
resource: src/services/plan.rs#L101-L105
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/services/plan/test_service_get_most_recent_plan
  - functions/src/tui/app/App/load_plan_for_viewing
  - functions/src/tui/app/App/check_and_load_plan
  - functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration
---

# Signature

`pub async fn get_most_recent_plan(&self, session_id: Uuid) -> Result<Option<PlanDocument>>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [test_service_get_most_recent_plan](../../../../../functions/src/services/plan/test_service_get_most_recent_plan.md)
- [load_plan_for_viewing](../../../../../functions/src/tui/app/App/load_plan_for_viewing.md)
- [check_and_load_plan](../../../../../functions/src/tui/app/App/check_and_load_plan.md)
- [test_get_most_recent_plan_integration](../../../../../functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration.md)