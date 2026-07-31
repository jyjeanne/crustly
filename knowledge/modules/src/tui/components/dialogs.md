---
type: Rust Module
title: dialogs
resource: src/tui/components/dialogs/mod.rs#L1-L171
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-plan-autorunmode-planmodestate-plantask-taskstatus
  - external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-clear-paragraph-wrap-frame
  member_of:
  - packages/crustly
---

# Contains

- [render_auto_exec_progress](../../../../functions/src/tui/components/dialogs/render_auto_exec_progress.md)
- [render_crash_recovery_dialog](../../../../functions/src/tui/components/dialogs/render_crash_recovery_dialog.md)
- [render_policy_denial](../../../../functions/src/tui/components/dialogs/render_policy_denial.md)
- [centered_rect](../../../../functions/src/tui/components/dialogs/centered_rect.md)

# Imports

- `crate::plan::{AutoRunMode, PlanModeState, PlanTask, TaskStatus}`
- `ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
}`

# Member of

- [crustly](../../../../packages/crustly.md)