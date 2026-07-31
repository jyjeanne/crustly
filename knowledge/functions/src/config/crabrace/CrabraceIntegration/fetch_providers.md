---
type: Rust Method
title: fetch_providers
resource: src/config/crabrace.rs#L71-L76
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/config/crabrace/CrabraceIntegration/get_provider
  - functions/src/config/crabrace/CrabraceIntegration/get_all_model_ids
  - functions/src/config/update/ProviderUpdater/update
---

# Signature

`pub async fn fetch_providers(&self) -> Result<Vec<Provider>>`

# Called by

- [get_provider](../../../../../functions/src/config/crabrace/CrabraceIntegration/get_provider.md)
- [get_all_model_ids](../../../../../functions/src/config/crabrace/CrabraceIntegration/get_all_model_ids.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)