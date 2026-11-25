---
type: Rust Function
title: transcript
resource: src/memory/extract.rs#L30-L43
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/extract/redact_and_omit
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn transcript(turns: &[Turn]) -> String`

# Calls

- [redact_and_omit](../../../../functions/src/memory/extract/redact_and_omit.md)

# Called by

- [long_user_messages_are_cut_and_answers_cut_shorter](../../../../functions/src/memory/extract/long_user_messages_are_cut_and_answers_cut_shorter.md)
- [long_code_is_omitted_and_short_code_is_kept](../../../../functions/src/memory/extract/long_code_is_omitted_and_short_code_is_kept.md)
- [secret_lines_are_redacted_before_the_model_sees_them](../../../../functions/src/memory/extract/secret_lines_are_redacted_before_the_model_sees_them.md)
- [extract](../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [render_chat](../../../../functions/src/tui/render/render_chat.md)