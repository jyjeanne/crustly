---
type: Rust Method
title: health_check
resource: src/config/crabrace.rs#L79-L84
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/crabrace/test_health_check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/ProviderUpdater/update
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn health_check(&self) -> Result<bool>`

# Calls

- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_health_check](../../../../../functions/src/config/crabrace/test_health_check.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)