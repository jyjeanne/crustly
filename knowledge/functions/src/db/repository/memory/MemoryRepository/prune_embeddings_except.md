---
type: Rust Method
title: prune_embeddings_except
resource: src/db/repository/memory.rs#L768-L776
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn prune_embeddings_except(&self, model: &str) -> Result<u64>`

# Called by

- [reindex](../../../../../../functions/src/memory/MemoryStore/reindex.md)
- [prune_keeps_only_the_current_model_and_counts_match](../../../../../../functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match.md)