---
type: Rust Function
title: chat_shows_pending_plan_banner_only_while_awaiting_approval
resource: src/tui/render.rs#L3801-L3818
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_shows_pending_plan_banner_only_while_awaiting_approval()`

# Calls

- [new](../../../../functions/src/plan/PlanDocument/new.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)