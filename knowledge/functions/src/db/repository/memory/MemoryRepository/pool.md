---
type: Rust Method
title: pool
resource: src/db/repository/memory.rs#L47-L49
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn pool(&self) -> SqlitePool`

# Called by

- [attach_embedder](../../../../../../functions/src/memory/MemoryStore/attach_embedder.md)