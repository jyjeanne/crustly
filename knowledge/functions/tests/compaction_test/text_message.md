---
type: Rust Function
title: text_message
resource: tests/compaction_test.rs#L29-L36
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/compaction_test/build_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn text_message(role: Role, text: &str) -> Message`

# Called by

- [build_context](../../../functions/tests/compaction_test/build_context.md)