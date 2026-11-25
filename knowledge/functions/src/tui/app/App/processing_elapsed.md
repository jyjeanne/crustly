---
type: Rust Method
title: processing_elapsed
resource: src/tui/app.rs#L749-L753
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/render_processing_indicator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_status_bar
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn processing_elapsed(&self) -> Option<std::time::Duration>`

# Called by

- [render_processing_indicator](../../../../../functions/src/tui/render/render_processing_indicator.md)
- [render_status_bar](../../../../../functions/src/tui/render/render_status_bar.md)