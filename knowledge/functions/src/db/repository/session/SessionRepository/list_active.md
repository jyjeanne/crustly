---
type: Rust Method
title: list_active
resource: src/db/repository/session.rs#L149-L158
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_active(&self) -> Result<Vec<Session>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)