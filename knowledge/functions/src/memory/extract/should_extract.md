---
type: Rust Function
title: should_extract
resource: src/memory/extract.rs#L88-L93
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/extract/extraction_runs_after_enough_messages_or_at_session_end
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn should_extract(new_user_messages: usize, every_n: usize, session_end: bool) -> bool`

# Called by

- [extraction_runs_after_enough_messages_or_at_session_end](../../../../functions/src/memory/extract/extraction_runs_after_enough_messages_or_at_session_end.md)
- [extract](../../../../functions/src/memory/extraction/MemoryStore/extract.md)