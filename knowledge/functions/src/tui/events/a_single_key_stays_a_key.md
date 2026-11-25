---
type: Rust Function
title: a_single_key_stays_a_key
resource: src/tui/events.rs#L772-L778
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/press
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn a_single_key_stays_a_key()`

# Calls

- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)
- [press](../../../../functions/src/tui/events/press.md)