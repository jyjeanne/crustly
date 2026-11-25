---
type: Rust Function
title: assistant_message
resource: src/tui/render.rs#L3898-L3912
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/chat_follows_the_end_of_a_reply_whose_paragraphs_wrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/scrolling_up_hides_the_end_and_says_how_to_return
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/an_empty_chat_shows_ferris_and_a_welcome
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/ferris_works_while_the_model_thinks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn assistant_message(content: &str) -> DisplayMessage`

# Called by

- [chat_follows_the_end_of_a_reply_whose_paragraphs_wrap](../../../../functions/src/tui/render/chat_follows_the_end_of_a_reply_whose_paragraphs_wrap.md)
- [scrolling_up_hides_the_end_and_says_how_to_return](../../../../functions/src/tui/render/scrolling_up_hides_the_end_and_says_how_to_return.md)
- [an_empty_chat_shows_ferris_and_a_welcome](../../../../functions/src/tui/render/an_empty_chat_shows_ferris_and_a_welcome.md)
- [ferris_works_while_the_model_thinks](../../../../functions/src/tui/render/ferris_works_while_the_model_thinks.md)