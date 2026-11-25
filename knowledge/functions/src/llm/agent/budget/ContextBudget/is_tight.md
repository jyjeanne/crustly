---
type: Rust Method
title: is_tight
resource: src/llm/agent/budget.rs#L156-L158
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/min_answer_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/history_crowds_answer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_tight(&self) -> bool`

# Calls

- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [min_answer_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/min_answer_tokens.md)

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [history_crowds_answer](../../../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)
- [full_tool_set_leaves_little_of_an_8k_window](../../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [history_crowds_the_answer_only_when_dropping_it_would_help](../../../../../../functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help.md)
- [make_room](../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)