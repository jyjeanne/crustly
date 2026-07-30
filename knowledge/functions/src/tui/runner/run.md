---
type: Rust Function
title: run
resource: src/tui/runner.rs#L27-L79
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/set_kitty_keyboard_protocol_active
  - functions/src/tui/runner/run_inner
---

# Signature

`pub async fn run(mut app: App) -> Result<()>`

# Calls

- [set_kitty_keyboard_protocol_active](../../../../functions/src/tui/app/App/set_kitty_keyboard_protocol_active.md)
- [run_inner](../../../../functions/src/tui/runner/run_inner.md)