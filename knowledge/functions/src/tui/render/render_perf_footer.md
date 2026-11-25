---
type: Rust Function
title: render_perf_footer
resource: src/tui/render.rs#L353-L386
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn render_perf_footer(msg: &super::app::DisplayMessage) -> Option<Line<'static>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [render_message_lines](../../../../functions/src/tui/render/render_message_lines.md)