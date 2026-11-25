---
type: Rust Method
title: get_most_recent_plan
resource: src/services/plan.rs#L101-L105
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/PlanService/find_by_session_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/test_service_get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_plan_for_viewing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_and_load_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_most_recent_plan(&self, session_id: Uuid) -> Result<Option<PlanDocument>>`

# Calls

- [find_by_session_id](../../../../../functions/src/services/plan/PlanService/find_by_session_id.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [test_service_get_most_recent_plan](../../../../../functions/src/services/plan/test_service_get_most_recent_plan.md)
- [load_plan_for_viewing](../../../../../functions/src/tui/app/App/load_plan_for_viewing.md)
- [check_and_load_plan](../../../../../functions/src/tui/app/App/check_and_load_plan.md)
- [test_get_most_recent_plan_integration](../../../../../functions/tests/plan_mode_integration_test/test_get_most_recent_plan_integration.md)