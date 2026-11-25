---
type: Rust Function
title: press
resource: src/tui/events.rs#L741-L743
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/events/typed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/releases_are_dropped_without_splitting_a_paste
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/a_single_key_stays_a_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/shifted_and_altgr_characters_are_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/a_crlf_pair_is_one_line_break
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/bracketed_pastes_and_resizes_keep_their_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn press(code: KeyCode) -> Event`

# Called by

- [typed](../../../../functions/src/tui/events/typed.md)
- [shortcuts_and_navigation_split_a_paste_and_pass_through](../../../../functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through.md)
- [releases_are_dropped_without_splitting_a_paste](../../../../functions/src/tui/events/releases_are_dropped_without_splitting_a_paste.md)
- [a_single_key_stays_a_key](../../../../functions/src/tui/events/a_single_key_stays_a_key.md)
- [shifted_and_altgr_characters_are_text](../../../../functions/src/tui/events/shifted_and_altgr_characters_are_text.md)
- [a_crlf_pair_is_one_line_break](../../../../functions/src/tui/events/a_crlf_pair_is_one_line_break.md)
- [bracketed_pastes_and_resizes_keep_their_order](../../../../functions/src/tui/events/bracketed_pastes_and_resizes_keep_their_order.md)