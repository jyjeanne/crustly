---
type: Rust Function
title: is_model_info
resource: src/tui/events.rs#L484-L486
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/key_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_model_info_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_model_info(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [test_model_info_key](../../../../functions/src/tui/events/test_model_info_key.md)