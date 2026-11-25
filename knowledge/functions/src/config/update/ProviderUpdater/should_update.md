---
type: Rust Method
title: should_update
resource: src/config/update.rs#L31-L47
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/update/ProviderUpdater/start_auto_update_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/test_should_update_when_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/update/test_should_update_when_never_updated
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn should_update(&self, config: &Config) -> bool`

# Called by

- [start_auto_update_loop](../../../../../functions/src/config/update/ProviderUpdater/start_auto_update_loop.md)
- [test_should_update_when_disabled](../../../../../functions/src/config/update/test_should_update_when_disabled.md)
- [test_should_update_when_never_updated](../../../../../functions/src/config/update/test_should_update_when_never_updated.md)