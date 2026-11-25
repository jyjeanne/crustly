---
type: Rust Method
title: finish_session
resource: src/memory/extraction.rs#L94-L114
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/extraction/MemoryStore/extract
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

# Called by

- [finish_memory](../../../../../functions/src/tui/app/App/finish_memory.md)