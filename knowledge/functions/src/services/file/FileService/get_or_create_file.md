---
type: Rust Method
title: get_or_create_file
resource: src/services/file.rs#L143-L156
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/file/FileService/find_file_by_path
  called_by:
  - functions/src/services/file/test_get_or_create_file
---

# Signature

`pub async fn get_or_create_file( &self, session_id: Uuid, path: PathBuf, content: Option<String>, ) -> Result<File>`

# Calls

- [find_file_by_path](../../../../../functions/src/services/file/FileService/find_file_by_path.md)

# Called by

- [test_get_or_create_file](../../../../../functions/src/services/file/test_get_or_create_file.md)