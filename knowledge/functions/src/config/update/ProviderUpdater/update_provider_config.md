---
type: Rust Method
title: update_provider_config
resource: src/config/update.rs#L109-L153
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/update/ProviderUpdater/update
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn update_provider_config(&self, config: &mut Config, provider: &Provider) -> bool`

# Called by

- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)