---
type: Rust Function
title: refused
resource: src/llm/agent/service/tool_round.rs#L49-L67
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/refuse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refused( tool_id: String, name: String, input: Value, message: String, ) -> (ContentBlock, ToolCallRecord)`

# Called by

- [refuse](../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/refuse.md)