---
type: Rust Method
title: fixed_overhead
resource: src/llm/agent/budget.rs#L109-L115
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/calibrated
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/history_crowds_answer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fixed_overhead(&self) -> u32`

# Calls

- [calibrated](../../../../../../functions/src/llm/agent/budget/ContextBudget/calibrated.md)

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [history_crowds_answer](../../../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [full_tool_set_leaves_little_of_an_8k_window](../../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [make_room](../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)