---
type: Rust Method
title: is_ready
resource: src/memory/code_knowledge.rs#L305-L307
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/ready_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_ready(&self) -> bool`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [resolve_text](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)