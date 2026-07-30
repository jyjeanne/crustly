---
type: Rust Method
title: delete_files_for_session
resource: src/services/file.rs#L118-L126
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/file/test_delete_files_for_session
---

# Signature

`pub async fn delete_files_for_session(&self, session_id: Uuid) -> Result<()>`

# Called by

- [test_delete_files_for_session](../../../../../functions/src/services/file/test_delete_files_for_session.md)