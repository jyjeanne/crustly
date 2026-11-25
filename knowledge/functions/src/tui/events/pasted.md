---
type: Rust Function
title: pasted
resource: src/tui/events.rs#L754-L762
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/events/keys_read_together_become_one_paste
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/releases_are_dropped_without_splitting_a_paste
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/shifted_and_altgr_characters_are_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/a_crlf_pair_is_one_line_break
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pasted(events: &[TuiEvent]) -> Vec<&str>`

# Called by

- [keys_read_together_become_one_paste](../../../../functions/src/tui/events/keys_read_together_become_one_paste.md)
- [releases_are_dropped_without_splitting_a_paste](../../../../functions/src/tui/events/releases_are_dropped_without_splitting_a_paste.md)
- [shifted_and_altgr_characters_are_text](../../../../functions/src/tui/events/shifted_and_altgr_characters_are_text.md)
- [a_crlf_pair_is_one_line_break](../../../../functions/src/tui/events/a_crlf_pair_is_one_line_break.md)