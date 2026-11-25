---
type: Rust Function
title: secret_lines_are_redacted_before_the_model_sees_them
resource: src/memory/extract.rs#L319-L324
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/extract/transcript
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/turn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn secret_lines_are_redacted_before_the_model_sees_them()`

# Calls

- [transcript](../../../../functions/src/memory/extract/transcript.md)
- [turn](../../../../functions/src/memory/extract/turn.md)