---
type: Rust Method
title: embedding_counts
resource: src/db/repository/memory.rs#L780-L793
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/embedding_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn embedding_counts(&self, model: &str) -> Result<(i64, i64)>`

# Called by

- [embedding_status](../../../../../../functions/src/memory/MemoryStore/embedding_status.md)
- [prune_keeps_only_the_current_model_and_counts_match](../../../../../../functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match.md)