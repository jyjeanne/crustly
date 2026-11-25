---
type: Rust Method
title: anchor_automatically
resource: src/memory/anchors.rs#L97-L113
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/anchors/MemoryStore/ready_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/set_anchors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn anchor_automatically(&self, id: Uuid) -> Result<Vec<String>>`

# Calls

- [ready_code](../../../../../functions/src/memory/anchors/MemoryStore/ready_code.md)
- [get](../../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [resolve_text](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)
- [bundle_time](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_time.md)
- [set_anchors](../../../../../functions/src/db/repository/memory/MemoryRepository/set_anchors.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [save_with](../../../../../functions/src/memory/MemoryStore/save_with.md)