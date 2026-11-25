---
type: Rust Method
title: approve
resource: src/memory/extraction.rs#L300-L324
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/pending_replaces
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/supersede
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/queue_embedding
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn approve( &self, id: Uuid, ) -> Result<std::result::Result<(), super::model::RejectReason>>`

# Calls

- [get](../../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [pending_replaces](../../../../../functions/src/db/repository/memory/MemoryRepository/pending_replaces.md)
- [approve](../../../../../functions/src/db/repository/memory/MemoryRepository/approve.md)
- [supersede](../../../../../functions/src/db/repository/memory/MemoryRepository/supersede.md)
- [changed](../../../../../functions/src/memory/MemoryStore/changed.md)
- [queue_embedding](../../../../../functions/src/memory/MemoryStore/queue_embedding.md)

# Called by

- [save_with](../../../../../functions/src/memory/MemoryStore/save_with.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../../../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [an_approved_memory_is_embedded](../../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)