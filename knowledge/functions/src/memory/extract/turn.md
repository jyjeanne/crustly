---
type: Rust Function
title: turn
resource: src/memory/extract.rs#L285-L290
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/extract/long_user_messages_are_cut_and_answers_cut_shorter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/long_code_is_omitted_and_short_code_is_kept
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/secret_lines_are_redacted_before_the_model_sees_them
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn turn(role: &str, text: &str) -> Turn`

# Called by

- [long_user_messages_are_cut_and_answers_cut_shorter](../../../../functions/src/memory/extract/long_user_messages_are_cut_and_answers_cut_shorter.md)
- [long_code_is_omitted_and_short_code_is_kept](../../../../functions/src/memory/extract/long_code_is_omitted_and_short_code_is_kept.md)
- [secret_lines_are_redacted_before_the_model_sees_them](../../../../functions/src/memory/extract/secret_lines_are_redacted_before_the_model_sees_them.md)