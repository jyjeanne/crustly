---
type: Rust Method
title: history_prev
resource: src/tui/app.rs#L614-L633
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_history_entry
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn history_prev(&mut self) -> bool`

# Calls

- [input_text](../../../../../functions/src/tui/app/App/input_text.md)
- [load_history_entry](../../../../../functions/src/tui/app/App/load_history_entry.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)