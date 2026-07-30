---
type: Rust Method
title: delete_file
resource: src/services/file.rs#L109-L115
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/file/test_delete_file
---

# Signature

`pub async fn delete_file(&self, id: Uuid) -> Result<()>`

# Called by

- [test_delete_file](../../../../../functions/src/services/file/test_delete_file.md)