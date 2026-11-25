---
type: Rust Method
title: complete_response
resource: src/tui/app.rs#L2531-L2624
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/check_task_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/PerfMetrics/tokens_per_second
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_and_load_plan
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete_response( &mut self, response: crate::llm::agent::AgentResponse, ) -> Result<()>`

# Calls

- [check_task_completion](../../../../../functions/src/tui/app/App/check_task_completion.md)
- [tokens_per_second](../../../../../functions/src/llm/provider/types/PerfMetrics/tokens_per_second.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [check_and_load_plan](../../../../../functions/src/tui/app/App/check_and_load_plan.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)