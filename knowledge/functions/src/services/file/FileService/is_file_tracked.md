---
type: Rust Method
title: is_file_tracked
resource: src/services/file.rs#L137-L140
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/file/FileService/find_file_by_path
  called_by:
  - functions/src/services/file/test_is_file_tracked
---

# Signature

`pub async fn is_file_tracked(&self, session_id: Uuid, path: &Path) -> Result<bool>`

# Calls

- [find_file_by_path](../../../../../functions/src/services/file/FileService/find_file_by_path.md)

# Called by

- [test_is_file_tracked](../../../../../functions/src/services/file/test_is_file_tracked.md)