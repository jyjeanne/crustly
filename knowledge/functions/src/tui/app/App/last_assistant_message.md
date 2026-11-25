---
type: Rust Method
title: last_assistant_message
resource: src/tui/app.rs#L545-L547
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/copy_last_response_to_clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/last_assistant_message_finds_most_recent_assistant_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn last_assistant_message(&self) -> Option<&DisplayMessage>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [copy_last_response_to_clipboard](../../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [last_assistant_message_finds_most_recent_assistant_reply](../../../../../functions/src/tui/app/last_assistant_message_finds_most_recent_assistant_reply.md)
- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)