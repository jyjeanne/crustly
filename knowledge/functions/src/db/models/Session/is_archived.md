---
type: Rust Method
title: is_archived
resource: src/db/models.rs#L239-L241
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/models/test_session_new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/test_session_archived
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_archived(&self) -> bool`

# Called by

- [test_session_new](../../../../../functions/src/db/models/test_session_new.md)
- [test_session_archived](../../../../../functions/src/db/models/test_session_archived.md)
- [test_session_archive](../../../../../functions/src/db/repository/session/test_session_archive.md)