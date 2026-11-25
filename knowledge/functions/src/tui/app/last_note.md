---
type: Rust Function
title: last_note
resource: src/tui/app.rs#L6033-L6038
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_lists_the_project_and_global_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_commands_say_so_when_memory_is_off
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn last_note(app: &App) -> String`

# Called by

- [remember_without_text_shows_usage_and_saves_nothing](../../../../functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing.md)
- [forget_asks_first_and_deletes_only_with_yes](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes.md)
- [memory_lists_the_project_and_global_memories](../../../../functions/src/tui/app/memory_lists_the_project_and_global_memories.md)
- [memory_commands_say_so_when_memory_is_off](../../../../functions/src/tui/app/memory_commands_say_so_when_memory_is_off.md)