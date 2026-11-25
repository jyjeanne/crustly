---
type: Rust Method
title: anchor_locations
resource: src/memory/anchors.rs#L227-L249
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
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/location
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn anchor_locations( &self, project_id: &ProjectId, memories: &[Memory], ) -> HashMap<Uuid, String>`

# Calls

- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [location](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/location.md)

# Called by

- [try_recall](../../../../../functions/src/memory/MemoryStore/try_recall.md)