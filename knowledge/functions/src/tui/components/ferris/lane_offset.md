---
type: Rust Function
title: lane_offset
resource: src/tui/components/ferris.rs#L65-L76
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/components/ferris/walking_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/ferris_walks_to_the_edge_and_back
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn lane_offset(tick: usize, lane: usize) -> usize`

# Called by

- [walking_lines](../../../../../functions/src/tui/components/ferris/walking_lines.md)
- [ferris_walks_to_the_edge_and_back](../../../../../functions/src/tui/components/ferris/ferris_walks_to_the_edge_and_back.md)