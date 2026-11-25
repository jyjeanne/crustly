---
type: Rust Method
title: handle_inline_code
resource: src/tui/markdown.rs#L183-L191
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_inline_code(&mut self, code: pulldown_cmark::CowStr<'_>)`

# Called by

- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)