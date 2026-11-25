---
type: Rust Method
title: start_auto_update_loop
resource: src/config/update.rs#L156-L181
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/update/ProviderUpdater/should_update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/ProviderUpdater/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn start_auto_update_loop(mut self, mut config: Config)`

# Calls

- [should_update](../../../../../functions/src/config/update/ProviderUpdater/should_update.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)