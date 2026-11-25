---
type: Rust Function
title: releases_are_dropped_without_splitting_a_paste
resource: src/tui/events.rs#L799-L810
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
  - target: functions/src/tui/events/pasted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn releases_are_dropped_without_splitting_a_paste()`

# Calls

- [press](../../../../functions/src/tui/events/press.md)
- [pasted](../../../../functions/src/tui/events/pasted.md)
- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)