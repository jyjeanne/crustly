---
type: Rust Function
title: render_crash_recovery_dialog
resource: src/tui/components/dialogs/mod.rs#L76-L122
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/components/dialogs/centered_rect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_crash_recovery_dialog( f: &mut Frame, area: Rect, plan_title: &str, resume_at: usize, total: usize, )`

# Calls

- [centered_rect](../../../../../functions/src/tui/components/dialogs/centered_rect.md)