---
type: Rust Function
title: ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking
resource: src/tui/app.rs#L4811-L4831
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)