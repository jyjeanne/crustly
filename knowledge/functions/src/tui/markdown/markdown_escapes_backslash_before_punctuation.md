---
type: Rust Function
title: markdown_escapes_backslash_before_punctuation
resource: src/tui/markdown.rs#L322-L329
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
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn markdown_escapes_backslash_before_punctuation()`

# Calls

- [rendered_text](../../../../functions/src/tui/markdown/rendered_text.md)
- [parse_markdown](../../../../functions/src/tui/markdown/parse_markdown.md)