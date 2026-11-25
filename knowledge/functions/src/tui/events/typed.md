---
type: Rust Function
title: typed
resource: src/tui/events.rs#L745-L752
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/press
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/keys_read_together_become_one_paste
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn typed(text: &str) -> Vec<Event>`

# Calls

- [press](../../../../functions/src/tui/events/press.md)

# Called by

- [keys_read_together_become_one_paste](../../../../functions/src/tui/events/keys_read_together_become_one_paste.md)
- [shortcuts_and_navigation_split_a_paste_and_pass_through](../../../../functions/src/tui/events/shortcuts_and_navigation_split_a_paste_and_pass_through.md)