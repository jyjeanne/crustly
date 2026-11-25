---
type: Rust Method
title: anchored_in_file
resource: src/memory/anchors.rs#L207-L223
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
  - target: functions/src/memory/code_knowledge/CodeKnowledge/file_concepts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchored_to
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn anchored_in_file( &self, project_id: &ProjectId, relative: &str, ) -> HashSet<Uuid>`

# Calls

- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [anchors_for](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)
- [file_concepts](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/file_concepts.md)
- [anchored_to](../../../../../functions/src/memory/anchors/MemoryStore/anchored_to.md)

# Called by

- [memories_for_path](../../../../../functions/src/memory/MemoryStore/memories_for_path.md)