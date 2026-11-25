---
type: Rust Method
title: edit
resource: src/memory/mod.rs#L672-L703
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/content_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/update_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/queue_embedding
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn edit( &self, id: Uuid, text: &str, actor: &str, ) -> Result<std::result::Result<(), RejectReason>>`

# Calls

- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [validate](../../../../functions/src/memory/model/validate.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)
- [content_hash](../../../../functions/src/memory/model/content_hash.md)
- [find_by_hash](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)
- [update_content](../../../../functions/src/db/repository/memory/MemoryRepository/update_content.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)
- [queue_embedding](../../../../functions/src/memory/MemoryStore/queue_embedding.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)