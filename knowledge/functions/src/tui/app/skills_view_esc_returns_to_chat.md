---
type: Rust Function
title: skills_view_esc_returns_to_chat
resource: src/tui/app.rs#L5078-L5085
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_skills_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn skills_view_esc_returns_to_chat()`

# Calls

- [handle_skills_key](../../../../functions/src/tui/app/App/handle_skills_key.md)
- [key](../../../../functions/src/tui/app/key.md)