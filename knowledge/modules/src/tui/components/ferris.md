---
type: Rust Module
title: ferris
resource: src/tui/components/ferris.rs#L1-L148
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/ratatui-style-color-modifier-style-text-line-span
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Mood](../../../../classes/src/tui/components/ferris/Mood.md)
- [frame](../../../../functions/src/tui/components/ferris/frame.md)
- [lane_offset](../../../../functions/src/tui/components/ferris/lane_offset.md)
- [walking_lines](../../../../functions/src/tui/components/ferris/walking_lines.md)
- [text](../../../../functions/src/tui/components/ferris/text.md)
- [every_frame_row_is_the_same_width](../../../../functions/src/tui/components/ferris/every_frame_row_is_the_same_width.md)
- [ferris_blinks_and_waves](../../../../functions/src/tui/components/ferris/ferris_blinks_and_waves.md)
- [ferris_walks_to_the_edge_and_back](../../../../functions/src/tui/components/ferris/ferris_walks_to_the_edge_and_back.md)
- [walking_lines_fill_the_lane_exactly](../../../../functions/src/tui/components/ferris/walking_lines_fill_the_lane_exactly.md)

# Imports

- `ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)