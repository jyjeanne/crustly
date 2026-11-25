---
type: Rust Method
title: connect_file
resource: src/db/mod.rs#L212-L215
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn connect_file<P: AsRef<Path>>(path: P) -> Result<Self>`

# Calls

- [connect](../../../../../functions/src/db/Database/connect.md)