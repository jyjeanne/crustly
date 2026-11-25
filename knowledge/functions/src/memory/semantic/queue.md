---
type: Rust Function
title: queue
resource: src/memory/semantic.rs#L203-L205
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn queue() -> (mpsc::Sender<Uuid>, mpsc::Receiver<Uuid>)`

# Called by

- [attach_embedder](../../../../functions/src/memory/MemoryStore/attach_embedder.md)