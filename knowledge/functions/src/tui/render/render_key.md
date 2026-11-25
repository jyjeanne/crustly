---
type: Rust Function
title: render_key
resource: src/tui/render.rs#L151-L162
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/MessageLineCache/lines_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_key(msg: &DisplayMessage, model_name: &str) -> u64`

# Called by

- [lines_for](../../../../functions/src/tui/render/MessageLineCache/lines_for.md)