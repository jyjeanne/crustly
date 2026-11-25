---
type: Rust Method
title: ready_code
resource: src/memory/anchors.rs#L89-L91
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/is_ready
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_automatically
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchored_in_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_locations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ready_code(&self) -> Option<&Arc<CodeKnowledge>>`

# Calls

- [is_ready](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/is_ready.md)

# Called by

- [anchor_automatically](../../../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [set_anchors](../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [anchor_hits](../../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)
- [anchored_in_file](../../../../../functions/src/memory/anchors/MemoryStore/anchored_in_file.md)
- [anchor_locations](../../../../../functions/src/memory/anchors/MemoryStore/anchor_locations.md)
- [drifted](../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)