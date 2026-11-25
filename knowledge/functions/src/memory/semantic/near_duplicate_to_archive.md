---
type: Rust Function
title: near_duplicate_to_archive
resource: src/memory/semantic.rs#L285-L300
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/loser
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn near_duplicate_to_archive<'a>(a: &'a Memory, b: &'a Memory) -> Option<&'a Memory>`

# Called by

- [consolidate](../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [archive_near_duplicates](../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [loser](../../../../functions/src/memory/semantic/loser.md)