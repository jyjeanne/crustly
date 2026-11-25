---
type: Rust Function
title: resolve_memory
resource: src/cli/mod.rs#L1130-L1147
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn resolve_memory( store: &crate::memory::MemoryStore, prefix: &str, ) -> Result<crate::memory::model::Memory>`

# Calls

- [find](../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [cmd_memory](../../../functions/src/cli/cmd_memory.md)