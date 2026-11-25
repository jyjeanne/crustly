---
type: Rust Method
title: reindex
resource: src/memory/mod.rs#L168-L191
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/semantic/backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/prune_embeddings_except
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn reindex(&self, prune: bool) -> Result<(usize, u64)>`

# Calls

- [backfill](../../../../functions/src/memory/semantic/backfill.md)
- [prune_embeddings_except](../../../../functions/src/db/repository/memory/MemoryRepository/prune_embeddings_except.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)