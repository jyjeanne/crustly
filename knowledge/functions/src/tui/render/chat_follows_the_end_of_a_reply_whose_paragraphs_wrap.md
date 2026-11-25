---
type: Rust Function
title: chat_follows_the_end_of_a_reply_whose_paragraphs_wrap
resource: src/tui/render.rs#L3915-L3928
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/assistant_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_follows_the_end_of_a_reply_whose_paragraphs_wrap()`

# Calls

- [assistant_message](../../../../functions/src/tui/render/assistant_message.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)