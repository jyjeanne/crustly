---
type: Rust Method
title: update
resource: src/config/update.rs#L50-L106
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/crabrace/CrabraceIntegration/health_check
  - functions/src/config/crabrace/CrabraceIntegration/fetch_providers
  - functions/src/config/update/ProviderUpdater/update_provider_config
---

# Signature

`pub async fn update(&mut self, config: &mut Config) -> Result<UpdateResult>`

# Calls

- [health_check](../../../../../functions/src/config/crabrace/CrabraceIntegration/health_check.md)
- [fetch_providers](../../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)
- [update_provider_config](../../../../../functions/src/config/update/ProviderUpdater/update_provider_config.md)