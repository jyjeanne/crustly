---
type: Rust Method
title: update_file_content
resource: src/services/file.rs#L94-L106
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/file/FileService/get_file_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/test_update_file_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_file_content(&self, id: Uuid, content: Option<String>) -> Result<()>`

# Calls

- [get_file_required](../../../../../functions/src/services/file/FileService/get_file_required.md)
- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/file/FileRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_update_file_content](../../../../../functions/src/services/file/test_update_file_content.md)