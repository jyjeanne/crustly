---
type: Rust Method
title: update_stats
resource: src/db/repository/session.rs#L204-L225
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_stats(&self, id: Uuid, token_delta: i32, cost_delta: f64) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)