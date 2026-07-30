---
type: Rust Method
title: should_update
resource: src/config/update.rs#L31-L47
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/config/update/ProviderUpdater/start_auto_update_loop
---

# Signature

`pub fn should_update(&self, config: &Config) -> bool`

# Called by

- [start_auto_update_loop](../../../../../functions/src/config/update/ProviderUpdater/start_auto_update_loop.md)