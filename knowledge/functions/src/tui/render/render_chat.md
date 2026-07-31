---
type: Rust Function
title: render_chat
resource: src/tui/render.rs#L431-L471
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/render/render_pending_plan_banner
  - functions/src/tui/render/render_message_lines
  - functions/src/tui/render/render_streaming_response
  - functions/src/tui/render/render_processing_indicator
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/render/compute_scroll_offset
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_chat(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_pending_plan_banner](../../../../functions/src/tui/render/render_pending_plan_banner.md)
- [render_message_lines](../../../../functions/src/tui/render/render_message_lines.md)
- [render_streaming_response](../../../../functions/src/tui/render/render_streaming_response.md)
- [render_processing_indicator](../../../../functions/src/tui/render/render_processing_indicator.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [compute_scroll_offset](../../../../functions/src/tui/render/compute_scroll_offset.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)