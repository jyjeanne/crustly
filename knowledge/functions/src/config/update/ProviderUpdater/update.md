---
type: Rust Method
title: update
resource: src/config/update.rs#L50-L106
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/health_check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/CrabraceIntegration/fetch_providers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/ProviderUpdater/update_provider_config
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/update/ProviderUpdater/start_auto_update_loop
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/ProviderUpdater/update_once
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update(&mut self, config: &mut Config) -> Result<UpdateResult>`

# Calls

- [health_check](../../../../../functions/src/config/crabrace/CrabraceIntegration/health_check.md)
- [fetch_providers](../../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)
- [update_provider_config](../../../../../functions/src/config/update/ProviderUpdater/update_provider_config.md)

# Called by

- [start_auto_update_loop](../../../../../functions/src/config/update/ProviderUpdater/start_auto_update_loop.md)
- [update_once](../../../../../functions/src/config/update/ProviderUpdater/update_once.md)