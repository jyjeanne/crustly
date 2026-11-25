---
type: Rust Function
title: build_memory_store
resource: src/cli/mod.rs#L214-L244
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/with_code_knowledge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/start_code_knowledge_in_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_memory_store( pool: &sqlx::SqlitePool, config: &crate::config::Config, background: bool, provider: Option<&Arc<dyn crate::llm::provider::Provider>>, ) -> crate::memory::MemoryStore`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [with_code_knowledge](../../../functions/src/memory/anchors/MemoryStore/with_code_knowledge.md)
- [detect](../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [start_code_knowledge_in_background](../../../functions/src/memory/anchors/MemoryStore/start_code_knowledge_in_background.md)
- [embedder_from_config](../../../functions/src/memory/embedder_from_config.md)
- [attach_embedder](../../../functions/src/memory/MemoryStore/attach_embedder.md)

# Called by

- [memory_store](../../../functions/src/cli/memory_store.md)
- [cmd_memory](../../../functions/src/cli/cmd_memory.md)