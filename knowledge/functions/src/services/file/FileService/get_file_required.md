---
type: Rust Method
title: get_file_required
resource: src/services/file.rs#L55-L59
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/file/FileService/get_file
  called_by:
  - functions/src/services/file/FileService/update_file_content
  - functions/src/services/file/test_update_file_content
---

# Signature

`pub async fn get_file_required(&self, id: Uuid) -> Result<File>`

# Calls

- [get_file](../../../../../functions/src/services/file/FileService/get_file.md)

# Called by

- [update_file_content](../../../../../functions/src/services/file/FileService/update_file_content.md)
- [test_update_file_content](../../../../../functions/src/services/file/test_update_file_content.md)