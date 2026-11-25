---
type: Rust Function
title: split_notes
resource: src/memory/transfer.rs#L197-L218
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/split_finds_only_the_bullets_of_the_memories_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/a_file_without_the_section_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn split_notes(text: &str) -> NotesSplit`

# Called by

- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [split_finds_only_the_bullets_of_the_memories_section](../../../../functions/src/memory/transfer/split_finds_only_the_bullets_of_the_memories_section.md)
- [a_file_without_the_section_is_unchanged](../../../../functions/src/memory/transfer/a_file_without_the_section_is_unchanged.md)