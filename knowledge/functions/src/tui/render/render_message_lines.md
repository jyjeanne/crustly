---
type: Rust Function
title: render_message_lines
resource: src/tui/render.rs#L319-L374
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/render/render_thinking_block
  - functions/src/tui/markdown/parse_plain_text
  - functions/src/tui/markdown/parse_markdown
  - functions/src/tui/render/render_perf_footer
  called_by:
  - functions/src/tui/render/render_chat
  - functions/src/tui/render/message_header_timestamp_is_shown_in_local_time
---

# Signature

`fn render_message_lines(msg: &super::app::DisplayMessage, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [render_thinking_block](../../../../functions/src/tui/render/render_thinking_block.md)
- [parse_plain_text](../../../../functions/src/tui/markdown/parse_plain_text.md)
- [parse_markdown](../../../../functions/src/tui/markdown/parse_markdown.md)
- [render_perf_footer](../../../../functions/src/tui/render/render_perf_footer.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)
- [message_header_timestamp_is_shown_in_local_time](../../../../functions/src/tui/render/message_header_timestamp_is_shown_in_local_time.md)