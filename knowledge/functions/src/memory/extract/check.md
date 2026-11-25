---
type: Rust Function
title: check
resource: src/memory/extract.rs#L228-L279
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/model/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extract/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check(raw: Raw, session_end: bool, min_importance: u8) -> Option<Candidate>`

# Calls

- [validate](../../../../functions/src/memory/model/validate.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [parse](../../../../functions/src/memory/extract/parse.md)