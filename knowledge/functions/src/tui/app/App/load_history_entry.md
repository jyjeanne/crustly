---
type: Rust Method
title: load_history_entry
resource: src/tui/app.rs#L410-L414
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/set_input_text
  called_by:
  - functions/src/tui/app/App/history_prev
  - functions/src/tui/app/App/history_next
---

# Signature

`fn load_history_entry(&mut self, entry: &str)`

# Calls

- [set_input_text](../../../../../functions/src/tui/app/App/set_input_text.md)

# Called by

- [history_prev](../../../../../functions/src/tui/app/App/history_prev.md)
- [history_next](../../../../../functions/src/tui/app/App/history_next.md)