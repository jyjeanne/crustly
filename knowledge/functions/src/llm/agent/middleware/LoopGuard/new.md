---
type: Rust Method
title: new
resource: src/llm/agent/middleware.rs#L254-L256
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [loop_guard_stops_a_repeated_write_on_the_second_call](../../../../../../functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call.md)
- [loop_guard_lets_exploration_revisit_a_path](../../../../../../functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path.md)
- [loop_guard_stops_drift_but_not_silent_plan_building](../../../../../../functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)