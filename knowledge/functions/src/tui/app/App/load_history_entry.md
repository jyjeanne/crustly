---
type: Rust Method
title: load_history_entry
resource: src/tui/app.rs#L604-L608
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/set_input_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/history_prev
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/history_next
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_history_entry(&mut self, entry: &str)`

# Calls

- [set_input_text](../../../../../functions/src/tui/app/App/set_input_text.md)

# Called by

- [history_prev](../../../../../functions/src/tui/app/App/history_prev.md)
- [history_next](../../../../../functions/src/tui/app/App/history_next.md)