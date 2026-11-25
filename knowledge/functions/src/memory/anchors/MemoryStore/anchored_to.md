---
type: Rust Method
title: anchored_to
resource: src/memory/anchors.rs#L155-L171
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchored_in_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn anchored_to( &self, project_id: &ProjectId, concepts: &HashSet<String>, ) -> Result<HashSet<Uuid>>`

# Calls

- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)

# Called by

- [anchor_hits](../../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)
- [anchored_in_file](../../../../../functions/src/memory/anchors/MemoryStore/anchored_in_file.md)