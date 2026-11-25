---
type: Rust Function
title: render_block
resource: src/memory/recall.rs#L238-L240
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/recall/render_block_located
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/recall/block_has_sections_in_order_and_skips_empty_ones
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/empty_sections_are_left_out
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/a_memory_cannot_close_the_block_early
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/the_block_states_that_memories_are_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/no_block_when_nothing_is_selected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_block(selected: &[Memory], recent_work: &[String]) -> Option<String>`

# Calls

- [render_block_located](../../../../functions/src/memory/recall/render_block_located.md)

# Called by

- [block_has_sections_in_order_and_skips_empty_ones](../../../../functions/src/memory/recall/block_has_sections_in_order_and_skips_empty_ones.md)
- [empty_sections_are_left_out](../../../../functions/src/memory/recall/empty_sections_are_left_out.md)
- [a_memory_cannot_close_the_block_early](../../../../functions/src/memory/recall/a_memory_cannot_close_the_block_early.md)
- [the_block_states_that_memories_are_data](../../../../functions/src/memory/recall/the_block_states_that_memories_are_data.md)
- [no_block_when_nothing_is_selected](../../../../functions/src/memory/recall/no_block_when_nothing_is_selected.md)