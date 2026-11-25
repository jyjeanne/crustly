---
type: Rust Function
title: an_empty_chat_shows_ferris_and_a_welcome
resource: src/tui/render.rs#L3951-L3963
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/assistant_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_empty_chat_shows_ferris_and_a_welcome()`

# Calls

- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)
- [assistant_message](../../../../functions/src/tui/render/assistant_message.md)