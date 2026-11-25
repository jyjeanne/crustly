---
type: Rust Function
title: is_copy_response
resource: src/tui/events.rs#L500-L502
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/key_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_copy_response_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_copy_response(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [test_copy_response_key](../../../../functions/src/tui/events/test_copy_response_key.md)