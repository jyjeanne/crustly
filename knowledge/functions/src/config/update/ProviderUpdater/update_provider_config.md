---
type: Rust Method
title: update_provider_config
resource: src/config/update.rs#L109-L153
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/config/update/ProviderUpdater/update
---

# Signature

`fn update_provider_config(&self, config: &mut Config, provider: &Provider) -> bool`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)