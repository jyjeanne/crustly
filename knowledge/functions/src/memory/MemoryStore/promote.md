---
type: Rust Method
title: promote
resource: src/memory/mod.rs#L735-L754
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/content_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/promote
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn promote(&self, id: Uuid) -> Result<std::result::Result<(), RejectReason>>`

# Calls

- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [content_hash](../../../../functions/src/memory/model/content_hash.md)
- [find_by_hash](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)
- [promote](../../../../functions/src/db/repository/memory/MemoryRepository/promote.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)