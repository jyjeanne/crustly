---
type: Rust Function
title: render_thinking_block
resource: src/tui/render.rs#L389-L425
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_message_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_thinking_block(msg: &super::app::DisplayMessage) -> Vec<Line<'static>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [render_message_lines](../../../../functions/src/tui/render/render_message_lines.md)