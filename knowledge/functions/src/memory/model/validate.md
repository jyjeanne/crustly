---
type: Rust Function
title: validate
resource: src/memory/model.rs#L266-L281
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extract/check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate_accepts_a_single_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate_rejects_empty_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate_rejects_long_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate_rejects_several_paragraphs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate_rejects_code_blocks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate(text: &str) -> Result<(), RejectReason>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [check](../../../../functions/src/memory/extract/check.md)
- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)
- [edit](../../../../functions/src/memory/MemoryStore/edit.md)
- [validate_accepts_a_single_fact](../../../../functions/src/memory/model/validate_accepts_a_single_fact.md)
- [validate_rejects_empty_text](../../../../functions/src/memory/model/validate_rejects_empty_text.md)
- [validate_rejects_long_text](../../../../functions/src/memory/model/validate_rejects_long_text.md)
- [validate_rejects_several_paragraphs](../../../../functions/src/memory/model/validate_rejects_several_paragraphs.md)
- [validate_rejects_code_blocks](../../../../functions/src/memory/model/validate_rejects_code_blocks.md)