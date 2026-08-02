---
type: Rust Method
title: get_provider
resource: src/config/crabrace.rs#L87-L90
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/crabrace/CrabraceIntegration/fetch_providers
  called_by:
  - functions/src/config/crabrace/CrabraceIntegration/is_provider_available
---

# Signature

`pub async fn get_provider(&self, provider_id: &str) -> Result<Option<Provider>>`

# Calls

- [fetch_providers](../../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)

# Called by

- [is_provider_available](../../../../../functions/src/config/crabrace/CrabraceIntegration/is_provider_available.md)