---
type: Rust Method
title: start
resource: src/memory/code_knowledge.rs#L333-L374
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/set_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/check_capabilities
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/spawn_start
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn start(&self) -> bool`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [set_status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/set_status.md)
- [check_capabilities](../../../../../functions/src/memory/code_knowledge/check_capabilities.md)

# Called by

- [spawn_start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/spawn_start.md)