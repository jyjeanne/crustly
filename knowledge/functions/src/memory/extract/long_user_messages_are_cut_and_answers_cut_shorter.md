---
type: Rust Function
title: long_user_messages_are_cut_and_answers_cut_shorter
resource: src/memory/extract.rs#L293-L306
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn long_user_messages_are_cut_and_answers_cut_shorter()`

# Calls

- [transcript](../../../../functions/src/memory/extract/transcript.md)
- [turn](../../../../functions/src/memory/extract/turn.md)