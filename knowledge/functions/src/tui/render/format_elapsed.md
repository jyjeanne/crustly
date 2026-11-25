---
type: Rust Function
title: format_elapsed
resource: src/tui/render.rs#L506-L513
visibility: private
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
  - target: functions/src/tui/render/elapsed_time_reads_naturally
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_elapsed(elapsed: std::time::Duration) -> String`

# Called by

- [render_processing_indicator](../../../../functions/src/tui/render/render_processing_indicator.md)
- [render_status_bar](../../../../functions/src/tui/render/render_status_bar.md)
- [elapsed_time_reads_naturally](../../../../functions/src/tui/render/elapsed_time_reads_naturally.md)