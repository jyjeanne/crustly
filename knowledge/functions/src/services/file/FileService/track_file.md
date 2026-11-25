---
type: Rust Method
title: track_file
resource: src/services/file.rs#L25-L46
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/file/FileRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/FileRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/file/FileService/get_or_create_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn track_file( &self, session_id: Uuid, path: PathBuf, content: Option<String>, ) -> Result<File>`

# Calls

- [new](../../../../../functions/src/db/repository/file/FileRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [create](../../../../../functions/src/db/repository/file/FileRepository/create.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [get_or_create_file](../../../../../functions/src/services/file/FileService/get_or_create_file.md)