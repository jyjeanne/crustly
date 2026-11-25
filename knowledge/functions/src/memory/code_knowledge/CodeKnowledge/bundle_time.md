---
type: Rust Method
title: bundle_time
resource: src/memory/code_knowledge.rs#L591-L593
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/newest_mtime
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_automatically
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn bundle_time(&self) -> Option<DateTime<Utc>>`

# Calls

- [newest_mtime](../../../../../functions/src/memory/code_knowledge/newest_mtime.md)

# Called by

- [anchor_automatically](../../../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [set_anchors](../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [drifted](../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)