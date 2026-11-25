---
type: Rust Function
title: coalesce_input
resource: src/tui/events.rs#L350-L376
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/typed_char
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/flush_run
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/read_terminal_events
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/keys_read_together_become_one_paste
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/a_single_key_stays_a_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/bracketed_pastes_and_resizes_keep_their_order
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

`pub(crate) fn coalesce_input(batch: Vec<crossterm::event::Event>) -> Vec<TuiEvent>`

# Calls

- [typed_char](../../../../functions/src/tui/events/typed_char.md)
- [flush_run](../../../../functions/src/tui/events/flush_run.md)

# Called by

- [read_terminal_events](../../../../functions/src/tui/events/read_terminal_events.md)
- [keys_read_together_become_one_paste](../../../../functions/src/tui/events/keys_read_together_become_one_paste.md)
- [a_single_key_stays_a_key](../../../../functions/src/tui/events/a_single_key_stays_a_key.md)
- [shortcuts_and_navigation_split_a_paste_and_pass_through](../../../../functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through.md)
- [bracketed_pastes_and_resizes_keep_their_order](../../../../functions/src/tui/events/bracketed_pastes_and_resizes_keep_their_order.md)
- [releases_are_dropped_without_splitting_a_paste](../../../../functions/src/tui/events/releases_are_dropped_without_splitting_a_paste.md)
- [shifted_and_altgr_characters_are_text](../../../../functions/src/tui/events/shifted_and_altgr_characters_are_text.md)
- [a_crlf_pair_is_one_line_break](../../../../functions/src/tui/events/a_crlf_pair_is_one_line_break.md)