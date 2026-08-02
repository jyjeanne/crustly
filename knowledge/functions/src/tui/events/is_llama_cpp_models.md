---
type: Rust Function
title: is_llama_cpp_models
resource: src/tui/events.rs#L350-L352
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`pub fn is_llama_cpp_models(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)