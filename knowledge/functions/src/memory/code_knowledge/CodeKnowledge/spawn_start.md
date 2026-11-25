---
type: Rust Method
title: spawn_start
resource: src/memory/code_knowledge.rs#L378-L385
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/start_code_knowledge_in_background
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn spawn_start(self: &Arc<Self>)`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/start.md)

# Called by

- [start_code_knowledge_in_background](../../../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge_in_background.md)