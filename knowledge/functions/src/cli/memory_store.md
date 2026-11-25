---
type: Rust Function
title: memory_store
resource: src/cli/mod.rs#L199-L208
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_store( pool: &sqlx::SqlitePool, config: &crate::config::Config, provider: &Arc<dyn crate::llm::provider::Provider>, ) -> Option<Arc<crate::memory::MemoryStore>>`

# Calls

- [build_memory_store](../../../functions/src/cli/build_memory_store.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)