---
type: Rust Method
title: get_file
resource: src/services/file.rs#L49-L52
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/file/FileService/get_file_required
  - functions/src/services/file/test_get_file
  - functions/src/services/file/test_delete_file
---

# Signature

`pub async fn get_file(&self, id: Uuid) -> Result<Option<File>>`

# Called by

- [get_file_required](../../../../../functions/src/services/file/FileService/get_file_required.md)
- [test_get_file](../../../../../functions/src/services/file/test_get_file.md)
- [test_delete_file](../../../../../functions/src/services/file/test_delete_file.md)