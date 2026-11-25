---
type: Rust Function
title: recall
resource: tests/memory_semantic_test.rs#L128-L130
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_semantic_test/recall_in_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall(store: &MemoryStore, project: &ProjectIdentity, message: &str) -> String`

# Calls

- [recall_in_window](../../../functions/tests/memory_semantic_test/recall_in_window.md)