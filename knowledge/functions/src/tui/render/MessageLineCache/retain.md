---
type: Rust Method
title: retain
resource: src/tui/render.rs#L141-L147
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn retain(&self, messages: &[DisplayMessage])`

# Called by

- [execute](../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [render_chat](../../../../../functions/src/tui/render/render_chat.md)