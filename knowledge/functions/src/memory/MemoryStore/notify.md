---
type: Rust Method
title: notify
resource: src/memory/mod.rs#L835-L848
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn notify(&self, scope: &MemoryScope, category: &str, id: Uuid, content: &str)`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)