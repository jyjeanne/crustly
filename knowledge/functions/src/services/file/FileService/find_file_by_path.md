---
type: Rust Method
title: find_file_by_path
resource: src/services/file.rs#L70-L75
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/file/FileRepository/find_by_path
  called_by:
  - functions/src/services/file/FileService/is_file_tracked
  - functions/src/services/file/FileService/get_or_create_file
  - functions/src/services/file/test_find_file_by_path
---

# Signature

`pub async fn find_file_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>`

# Calls

- [find_by_path](../../../../../functions/src/db/repository/file/FileRepository/find_by_path.md)

# Called by

- [is_file_tracked](../../../../../functions/src/services/file/FileService/is_file_tracked.md)
- [get_or_create_file](../../../../../functions/src/services/file/FileService/get_or_create_file.md)
- [test_find_file_by_path](../../../../../functions/src/services/file/test_find_file_by_path.md)