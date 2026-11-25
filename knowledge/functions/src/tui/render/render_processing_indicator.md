---
type: Rust Function
title: render_processing_indicator
resource: src/tui/render.rs#L517-L552
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/processing_elapsed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/walking_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/format_elapsed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_processing_indicator(app: &App, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [processing_elapsed](../../../../functions/src/tui/app/App/processing_elapsed.md)
- [walking_lines](../../../../functions/src/tui/components/ferris/walking_lines.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [format_elapsed](../../../../functions/src/tui/render/format_elapsed.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)