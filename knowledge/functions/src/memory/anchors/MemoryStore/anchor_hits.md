---
type: Rust Method
title: anchor_hits
resource: src/memory/anchors.rs#L177-L203
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/anchors/MemoryStore/ready_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/neighbours
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchored_to
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn anchor_hits(&self, project_id: &ProjectId, message: &str) -> AnchorHits`

# Calls

- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)
- [resolve_text](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)
- [neighbours](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/neighbours.md)
- [anchored_to](../../../../../functions/src/memory/anchors/MemoryStore/anchored_to.md)

# Called by

- [try_recall](../../../../../functions/src/memory/MemoryStore/try_recall.md)