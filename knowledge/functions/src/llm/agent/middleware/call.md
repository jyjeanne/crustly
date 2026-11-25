---
type: Rust Function
title: call
resource: src/llm/agent/middleware.rs#L571-L577
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn call(name: &str, input: Value, caps: Vec<ToolCapability>) -> ToolCallView`

# Called by

- [loop_guard_stops_a_repeated_write_on_the_second_call](../../../../../functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call.md)
- [loop_guard_lets_exploration_revisit_a_path](../../../../../functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path.md)
- [loop_guard_stops_drift_but_not_silent_plan_building](../../../../../functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)