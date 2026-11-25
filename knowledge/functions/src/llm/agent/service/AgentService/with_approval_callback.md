---
type: Rust Method
title: with_approval_callback
resource: src/llm/agent/service.rs#L750-L753
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_approval_callback(mut self, callback: Option<ApprovalCallback>) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [approval_callback_sees_the_canonical_tool_name](../../../../../../functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name.md)
- [cancelling_during_an_approval_prompt_ends_the_request](../../../../../../functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)