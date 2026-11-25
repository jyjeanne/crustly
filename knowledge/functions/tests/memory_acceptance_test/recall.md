---
type: Rust Function
title: recall
resource: tests/memory_acceptance_test.rs#L56-L73
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall( store: &MemoryStore, project: &ProjectIdentity, message: &str, first: bool, ) -> String`

# Calls

- [recall](../../../functions/src/memory/MemoryStore/recall.md)