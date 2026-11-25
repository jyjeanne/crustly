---
type: Rust Function
title: visible_memories
resource: src/tui/app.rs#L69-L82
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/clamp_memory_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_memory_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/scope_and_category_filters_narrow_the_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/search_filters_by_words_in_the_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_on_y
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn visible_memories(view: &MemoryView) -> Vec<&crate::memory::model::Memory>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [clamp_memory_selection](../../../../functions/src/tui/app/App/clamp_memory_selection.md)
- [load_memory_history](../../../../functions/src/tui/app/App/load_memory_history.md)
- [handle_memory_key](../../../../functions/src/tui/app/App/handle_memory_key.md)
- [selected_memory](../../../../functions/src/tui/app/App/selected_memory.md)
- [scope_and_category_filters_narrow_the_list](../../../../functions/src/tui/app/scope_and_category_filters_narrow_the_list.md)
- [search_filters_by_words_in_the_content](../../../../functions/src/tui/app/search_filters_by_words_in_the_content.md)
- [forget_asks_first_and_deletes_only_on_y](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_on_y.md)
- [memories_opens_the_screen_with_pending_memories_first](../../../../functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first.md)
- [render_memory](../../../../functions/src/tui/render/render_memory.md)