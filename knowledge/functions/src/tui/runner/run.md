---
type: Rust Function
title: run
resource: src/tui/runner.rs#L27-L84
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/set_kitty_keyboard_protocol_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn run(mut app: App) -> Result<()>`

# Calls

- [set_kitty_keyboard_protocol_active](../../../../functions/src/tui/app/App/set_kitty_keyboard_protocol_active.md)
- [run_inner](../../../../functions/src/tui/runner/run_inner.md)
- [finish_memory](../../../../functions/src/tui/app/App/finish_memory.md)