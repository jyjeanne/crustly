---
type: Rust Function
title: render_message_lines
resource: src/tui/render.rs#L429-L484
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/render_thinking_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/parse_plain_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_perf_footer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/MessageLineCache/lines_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/message_header_timestamp_is_shown_in_local_time
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_message_lines(msg: &super::app::DisplayMessage, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [render_thinking_block](../../../../functions/src/tui/render/render_thinking_block.md)
- [parse_plain_text](../../../../functions/src/tui/markdown/parse_plain_text.md)
- [parse_markdown](../../../../functions/src/tui/markdown/parse_markdown.md)
- [render_perf_footer](../../../../functions/src/tui/render/render_perf_footer.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [lines_for](../../../../functions/src/tui/render/MessageLineCache/lines_for.md)
- [message_header_timestamp_is_shown_in_local_time](../../../../functions/src/tui/render/message_header_timestamp_is_shown_in_local_time.md)