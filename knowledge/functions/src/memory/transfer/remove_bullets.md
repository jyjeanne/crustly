---
type: Rust Function
title: remove_bullets
resource: src/memory/transfer.rs#L223-L271
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/remove_drops_only_the_named_bullets_and_keeps_the_rest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/the_heading_goes_when_no_bullet_is_left
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/a_file_without_the_section_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn remove_bullets(text: &str, remove: &[String]) -> String`

# Calls

- [skip](../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [remove_drops_only_the_named_bullets_and_keeps_the_rest](../../../../functions/src/memory/transfer/remove_drops_only_the_named_bullets_and_keeps_the_rest.md)
- [the_heading_goes_when_no_bullet_is_left](../../../../functions/src/memory/transfer/the_heading_goes_when_no_bullet_is_left.md)
- [a_file_without_the_section_is_unchanged](../../../../functions/src/memory/transfer/a_file_without_the_section_is_unchanged.md)
- [removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines](../../../../functions/tests/memory_minor_fixes_test/removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines.md)