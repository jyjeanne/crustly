---
type: Rust Method
title: list_archived
resource: src/db/repository/session.rs#L161-L170
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_archived(&self) -> Result<Vec<Session>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)