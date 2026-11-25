---
type: Rust Method
title: available
resource: src/llm/agent/budget.rs#L123-L125
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/used
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/completion_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/calibration_scales_the_estimate_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn available(&self) -> u32`

# Calls

- [used](../../../../../../functions/src/llm/agent/budget/ContextBudget/used.md)

# Called by

- [is_full](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [completion_budget](../../../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)
- [is_tight](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [full_tool_set_leaves_little_of_an_8k_window](../../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [distinguishes_prompt_overflow_from_a_clamped_full_window](../../../../../../functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window.md)
- [calibration_scales_the_estimate_up](../../../../../../functions/src/llm/agent/budget/calibration_scales_the_estimate_up.md)
- [make_room](../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)