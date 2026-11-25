---
type: Rust Method
title: update_once
resource: src/config/update.rs#L184-L188
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/ProviderUpdater/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/ProviderUpdater/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update_once(config: &mut Config) -> Result<UpdateResult>`

# Calls

- [new](../../../../../functions/src/config/crabrace/CrabraceIntegration/new.md)
- [new](../../../../../functions/src/config/update/ProviderUpdater/new.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)