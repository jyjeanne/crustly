---
type: Rust Function
title: compaction_of_one_long_request_keeps_the_task_and_latest_rounds
resource: src/llm/agent/compaction.rs#L439-L465
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/pool_with_compaction_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/effective_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/is_plain_user_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/tool_results_are_paired
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn compaction_of_one_long_request_keeps_the_task_and_latest_rounds()`

# Calls

- [pool_with_compaction_table](../../../../../functions/src/llm/agent/compaction/pool_with_compaction_table.md)
- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [tool_round](../../../../../functions/src/llm/agent/compaction/tool_round.md)
- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [effective_system_prompt](../../../../../functions/src/llm/agent/context/AgentContext/effective_system_prompt.md)
- [is_plain_user_message](../../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)
- [tool_results_are_paired](../../../../../functions/src/llm/agent/compaction/tool_results_are_paired.md)