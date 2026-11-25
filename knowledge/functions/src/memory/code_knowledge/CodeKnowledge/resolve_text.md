---
type: Rust Method
title: resolve_text
resource: src/memory/code_knowledge.rs#L467-L483
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/is_ready
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/identifiers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_automatically
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn resolve_text(&self, text: &str) -> Vec<String>`

# Calls

- [is_ready](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/is_ready.md)
- [identifiers](../../../../../functions/src/memory/code_knowledge/identifiers.md)
- [resolve](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve.md)

# Called by

- [anchor_automatically](../../../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [anchor_hits](../../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)