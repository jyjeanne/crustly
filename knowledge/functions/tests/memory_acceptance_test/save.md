---
type: Rust Function
title: save
resource: tests/memory_acceptance_test.rs#L34-L53
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

`async fn save( store: &MemoryStore, scope: MemoryScope, content: &str, category: MemoryCategory, ) -> SaveOutcome`

# Calls

- [save](../../../functions/src/memory/MemoryStore/save.md)