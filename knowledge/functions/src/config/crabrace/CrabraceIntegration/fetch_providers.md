---
type: Rust Method
title: fetch_providers
resource: src/config/crabrace.rs#L71-L76
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/crabrace/CrabraceIntegration/get_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/CrabraceIntegration/get_all_model_ids
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/ProviderUpdater/update
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fetch_providers(&self) -> Result<Vec<Provider>>`

# Calls

- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [get_provider](../../../../../functions/src/config/crabrace/CrabraceIntegration/get_provider.md)
- [get_all_model_ids](../../../../../functions/src/config/crabrace/CrabraceIntegration/get_all_model_ids.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)