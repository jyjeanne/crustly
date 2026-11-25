---
type: Rust Function
title: elide_tool_results_leaves_short_output_alone
resource: src/llm/agent/compaction.rs#L403-L412
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
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
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn elide_tool_results_leaves_short_output_alone()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [tool_round](../../../../../functions/src/llm/agent/compaction/tool_round.md)
- [elide_tool_results](../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)