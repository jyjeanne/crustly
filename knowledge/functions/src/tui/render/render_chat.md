---
type: Rust Function
title: render_chat
resource: src/tui/render.rs#L597-L649
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/render_pending_plan_banner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_welcome
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/MessageLineCache/retain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/MessageLineCache/lines_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_streaming_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_processing_indicator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/transcript
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/fit_scroll
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_block
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_chat(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_pending_plan_banner](../../../../functions/src/tui/render/render_pending_plan_banner.md)
- [render_welcome](../../../../functions/src/tui/render/render_welcome.md)
- [retain](../../../../functions/src/tui/render/MessageLineCache/retain.md)
- [lines_for](../../../../functions/src/tui/render/MessageLineCache/lines_for.md)
- [render_streaming_response](../../../../functions/src/tui/render/render_streaming_response.md)
- [render_processing_indicator](../../../../functions/src/tui/render/render_processing_indicator.md)
- [transcript](../../../../functions/src/memory/extract/transcript.md)
- [fit_scroll](../../../../functions/src/tui/render/fit_scroll.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)
- [chat_block](../../../../functions/src/tui/render/chat_block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)