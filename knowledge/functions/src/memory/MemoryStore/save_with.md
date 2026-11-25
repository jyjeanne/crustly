---
type: Rust Method
title: save_with
resource: src/memory/mod.rs#L229-L344
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/model/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/actor_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/content_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/reinforce
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/touch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/supersede
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/set_pending_replaces
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/queue_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/notify
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryCategory/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn save_with( &self, new: NewMemory, status: MemoryStatus, user_evidence: bool, ) -> Result<SaveOutcome>`

# Calls

- [validate](../../../../functions/src/memory/model/validate.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)
- [actor_for](../../../../functions/src/memory/actor_for.md)
- [content_hash](../../../../functions/src/memory/model/content_hash.md)
- [find_by_hash](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)
- [reinforce](../../../../functions/src/db/repository/memory/MemoryRepository/reinforce.md)
- [touch](../../../../functions/src/db/repository/memory/MemoryRepository/touch.md)
- [approve](../../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [supersede](../../../../functions/src/db/repository/memory/MemoryRepository/supersede.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [insert](../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [set_pending_replaces](../../../../functions/src/db/repository/memory/MemoryRepository/set_pending_replaces.md)
- [queue_embedding](../../../../functions/src/memory/MemoryStore/queue_embedding.md)
- [notify](../../../../functions/src/memory/MemoryStore/notify.md)
- [as_str](../../../../functions/src/memory/model/MemoryCategory/as_str.md)

# Called by

- [extract](../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [save](../../../../functions/src/memory/MemoryStore/save.md)