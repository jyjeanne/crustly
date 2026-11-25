---
type: Rust Method
title: finish_session
resource: src/memory/extraction.rs#L94-L114
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/remove
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn finish_session(&self, session: Uuid, project: &ProjectIdentity) -> Result<usize>`

# Calls

- [extract](../../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [remove](../../../../../functions/src/memory/vector_index/VectorIndex/remove.md)

# Called by

- [finish_memory](../../../../../functions/src/tui/app/App/finish_memory.md)