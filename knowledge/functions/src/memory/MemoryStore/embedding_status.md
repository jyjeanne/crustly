---
type: Rust Method
title: embedding_status
resource: src/memory/mod.rs#L195-L208
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/embedding_counts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/wait_until_indexed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn embedding_status(&self) -> Result<Option<(String, i64, i64)>>`

# Calls

- [embedding_counts](../../../../functions/src/db/repository/memory/MemoryRepository/embedding_counts.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [reload_memory_rows](../../../../functions/src/tui/app/App/reload_memory_rows.md)
- [wait_until_indexed](../../../../functions/tests/memory_semantic_test/wait_until_indexed.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)