---
type: Rust Function
title: unmatched_memories_never_take_budget_when_others_compete
resource: src/memory/recall.rs#L416-L451
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/ranked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unmatched_memories_never_take_budget_when_others_compete()`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [ranked](../../../../functions/src/memory/recall/ranked.md)
- [select](../../../../functions/src/memory/recall/select.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)