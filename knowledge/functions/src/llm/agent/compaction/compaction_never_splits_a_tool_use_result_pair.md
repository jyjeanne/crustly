---
type: Rust Function
title: compaction_never_splits_a_tool_use_result_pair
resource: src/llm/agent/compaction.rs#L631-L703
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact
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

`async fn compaction_never_splits_a_tool_use_result_pair()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
- [is_plain_user_message](../../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)
- [tool_results_are_paired](../../../../../functions/src/llm/agent/compaction/tool_results_are_paired.md)