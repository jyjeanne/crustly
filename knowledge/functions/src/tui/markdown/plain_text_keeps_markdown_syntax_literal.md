---
type: Rust Function
title: plain_text_keeps_markdown_syntax_literal
resource: src/tui/markdown.rs#L307-L311
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/markdown/rendered_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/parse_plain_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plain_text_keeps_markdown_syntax_literal()`

# Calls

- [rendered_text](../../../../functions/src/tui/markdown/rendered_text.md)
- [parse_plain_text](../../../../functions/src/tui/markdown/parse_plain_text.md)