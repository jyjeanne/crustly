---
type: Rust Method
title: recount_tokens
resource: src/llm/agent/context.rs#L91-L97
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/estimate_message_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn recount_tokens(&mut self)`

# Calls

- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)

# Called by

- [elide_tool_results](../../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)