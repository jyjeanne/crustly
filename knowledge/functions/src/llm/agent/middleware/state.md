---
type: Rust Function
title: state
resource: src/llm/agent/middleware.rs#L561-L569
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/limits_finish_with_a_notice_after_the_last_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/limits_finish_on_tokens_and_wall_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/verification_gives_up_after_two_nudges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn state<'a>(round: usize, calls: &'a [ToolCallView], produced_text: bool) -> RunState<'a>`

# Called by

- [loop_guard_stops_drift_but_not_silent_plan_building](../../../../../functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building.md)
- [limits_finish_with_a_notice_after_the_last_round](../../../../../functions/src/llm/agent/middleware/limits_finish_with_a_notice_after_the_last_round.md)
- [limits_finish_on_tokens_and_wall_time](../../../../../functions/src/llm/agent/middleware/limits_finish_on_tokens_and_wall_time.md)
- [verification_nudges_after_an_edit_until_a_check_passes](../../../../../functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes.md)
- [loop_guard_stops_a_repeated_write_on_the_second_call](../../../../../functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call.md)
- [loop_guard_lets_exploration_revisit_a_path](../../../../../functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path.md)
- [verification_gives_up_after_two_nudges](../../../../../functions/src/llm/agent/middleware/verification_gives_up_after_two_nudges.md)
- [pipeline_consults_everyone_and_the_first_decision_wins](../../../../../functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins.md)