---
type: Rust Function
title: shifted_and_altgr_characters_are_text
resource: src/tui/events.rs#L813-L823
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/press
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/pasted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn shifted_and_altgr_characters_are_text()`

# Calls

- [press](../../../../functions/src/tui/events/press.md)
- [pasted](../../../../functions/src/tui/events/pasted.md)
- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)