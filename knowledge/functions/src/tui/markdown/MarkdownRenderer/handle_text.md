---
type: Rust Method
title: handle_text
resource: src/tui/markdown.rs#L171-L181
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_text(&mut self, text: pulldown_cmark::CowStr<'_>)`

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)