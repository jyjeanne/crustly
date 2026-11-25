---
type: Rust Function
title: walking_lines
resource: src/tui/components/ferris.rs#L80-L97
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/components/ferris/lane_offset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/frame
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/components/ferris/walking_lines_fill_the_lane_exactly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_processing_indicator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_welcome
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn walking_lines(tick: usize, lane: usize, mood: Mood) -> Vec<Line<'static>>`

# Calls

- [lane_offset](../../../../../functions/src/tui/components/ferris/lane_offset.md)
- [frame](../../../../../functions/src/tui/components/ferris/frame.md)

# Called by

- [walking_lines_fill_the_lane_exactly](../../../../../functions/src/tui/components/ferris/walking_lines_fill_the_lane_exactly.md)
- [render_processing_indicator](../../../../../functions/src/tui/render/render_processing_indicator.md)
- [render_welcome](../../../../../functions/src/tui/render/render_welcome.md)