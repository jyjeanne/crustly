---
type: Rust Function
title: keys_read_together_become_one_paste
resource: src/tui/events.rs#L765-L769
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/typed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/pasted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn keys_read_together_become_one_paste()`

# Calls

- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)
- [typed](../../../../functions/src/tui/events/typed.md)
- [pasted](../../../../functions/src/tui/events/pasted.md)