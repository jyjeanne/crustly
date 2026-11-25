---
type: Rust Method
title: finish
resource: src/tui/markdown.rs#L204-L212
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn finish(mut self) -> Vec<Line<'static>>`

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)