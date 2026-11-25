---
type: Rust Method
title: neighbours
resource: src/memory/code_knowledge.rs#L486-L496
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/explore
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn neighbours(&self, id: &str) -> Vec<String>`

# Calls

- [explore](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/explore.md)

# Called by

- [anchor_hits](../../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)