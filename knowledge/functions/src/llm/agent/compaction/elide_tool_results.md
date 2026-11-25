---
type: Rust Function
title: elide_tool_results
resource: src/llm/agent/compaction.rs#L142-L175
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/message_has_tool_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_at_char_boundary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/recount_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn elide_tool_results(ctx: &mut AgentContext, keep_recent: usize) -> usize`

# Calls

- [message_has_tool_result](../../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)
- [truncate_at_char_boundary](../../../../../functions/src/utils/truncate_at_char_boundary.md)
- [recount_tokens](../../../../../functions/src/llm/agent/context/AgentContext/recount_tokens.md)

# Called by

- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [elide_tool_results_leaves_short_output_alone](../../../../../functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone.md)
- [make_room](../../../../../functions/src/llm/agent/service/AgentService/make_room.md)