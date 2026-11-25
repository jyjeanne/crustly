---
type: Rust Function
title: is_llama_cpp_models
resource: src/tui/events.rs#L494-L496
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/key_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_llama_cpp_models_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_llama_cpp_models(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [test_llama_cpp_models_key](../../../../functions/src/tui/events/test_llama_cpp_models_key.md)