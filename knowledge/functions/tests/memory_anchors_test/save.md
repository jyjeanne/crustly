---
type: Rust Function
title: save
resource: tests/memory_anchors_test.rs#L27-L45
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save(store: &MemoryStore, project: &ProjectIdentity, content: &str) -> uuid::Uuid`

# Calls

- [save](../../../functions/src/memory/MemoryStore/save.md)