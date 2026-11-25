---
type: Rust Function
title: actor_for
resource: src/memory/mod.rs#L894-L902
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn actor_for(source: MemorySource) -> &'static str`

# Called by

- [save_with](../../../functions/src/memory/MemoryStore/save_with.md)