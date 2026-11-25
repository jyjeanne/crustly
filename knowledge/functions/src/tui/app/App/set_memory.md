---
type: Rust Method
title: set_memory
resource: src/tui/app.rs#L824-L826
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_test_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_memory(&mut self, store: Option<std::sync::Arc<crate::memory::MemoryStore>>)`

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
- [memory_test_app](../../../../../functions/src/tui/app/memory_test_app.md)
- [memory_screen_app](../../../../../functions/src/tui/app/memory_screen_app.md)
- [app_with_extraction](../../../../../functions/src/tui/app/app_with_extraction.md)