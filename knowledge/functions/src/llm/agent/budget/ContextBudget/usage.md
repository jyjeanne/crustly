---
type: Rust Method
title: usage
resource: src/llm/agent/budget.rs#L171-L180
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/used
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn usage(&self, core_tools_only: bool) -> ContextUsage`

# Calls

- [used](../../../../../../functions/src/llm/agent/budget/ContextBudget/used.md)
- [fixed_overhead](../../../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)
- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)

# Called by

- [full_tool_set_leaves_little_of_an_8k_window](../../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)