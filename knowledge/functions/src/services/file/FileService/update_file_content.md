---
type: Rust Method
title: update_file_content
resource: src/services/file.rs#L94-L106
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/file/FileService/get_file_required
  called_by:
  - functions/src/services/file/test_update_file_content
---

# Signature

`pub async fn update_file_content(&self, id: Uuid, content: Option<String>) -> Result<()>`

# Calls

- [get_file_required](../../../../../functions/src/services/file/FileService/get_file_required.md)

# Called by

- [test_update_file_content](../../../../../functions/src/services/file/test_update_file_content.md)