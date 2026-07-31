---
type: Rust Method
title: count_files_in_session
resource: src/services/file.rs#L129-L134
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/file/test_count_files_in_session
---

# Signature

`pub async fn count_files_in_session(&self, session_id: Uuid) -> Result<i64>`

# Called by

- [test_count_files_in_session](../../../../../functions/src/services/file/test_count_files_in_session.md)