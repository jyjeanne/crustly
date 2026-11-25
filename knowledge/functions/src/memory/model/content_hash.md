---
type: Rust Function
title: content_hash
resource: src/memory/model.rs#L255-L262
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/content_hash_matches_a_fixed_digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/content_hash_ignores_case_spacing_and_one_final_period
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn content_hash(text: &str) -> String`

# Called by

- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)
- [edit](../../../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../../../functions/src/memory/MemoryStore/promote.md)
- [content_hash_matches_a_fixed_digest](../../../../functions/src/memory/model/content_hash_matches_a_fixed_digest.md)
- [content_hash_ignores_case_spacing_and_one_final_period](../../../../functions/src/memory/model/content_hash_ignores_case_spacing_and_one_final_period.md)