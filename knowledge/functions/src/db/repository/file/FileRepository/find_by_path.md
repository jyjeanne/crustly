---
type: Rust Method
title: find_by_path
resource: src/db/repository/file.rs#L48-L59
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/file/FileService/find_file_by_path
---

# Signature

`pub async fn find_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>`

# Called by

- [find_file_by_path](../../../../../../functions/src/services/file/FileService/find_file_by_path.md)