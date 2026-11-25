---
type: Rust Function
title: frame
resource: src/tui/components/ferris.rs#L33-L61
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/components/ferris/walking_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/every_frame_row_is_the_same_width
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/ferris_blinks_and_waves
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn frame(tick: usize, mood: Mood) -> [String; HEIGHT]`

# Called by

- [walking_lines](../../../../../functions/src/tui/components/ferris/walking_lines.md)
- [every_frame_row_is_the_same_width](../../../../../functions/src/tui/components/ferris/every_frame_row_is_the_same_width.md)
- [ferris_blinks_and_waves](../../../../../functions/src/tui/components/ferris/ferris_blinks_and_waves.md)