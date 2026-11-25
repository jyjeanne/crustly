---
type: Rust Function
title: elide_tool_results_keeps_recent_output_and_frees_tokens
resource: src/llm/agent/compaction.rs#L371-L400
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
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/tool_results_are_paired
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn elide_tool_results_keeps_recent_output_and_frees_tokens()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [tool_round](../../../../../functions/src/llm/agent/compaction/tool_round.md)
- [elide_tool_results](../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)
- [tool_results_are_paired](../../../../../functions/src/llm/agent/compaction/tool_results_are_paired.md)