---
type: Rust Method
title: new
resource: src/config/update.rs#L23-L28
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/update/ProviderUpdater/update_once
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/test_should_update_when_disabled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/test_should_update_when_never_updated
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(crabrace: CrabraceIntegration) -> Self`

# Called by

- [update_once](../../../../../functions/src/config/update/ProviderUpdater/update_once.md)
- [test_should_update_when_disabled](../../../../../functions/src/config/update/test_should_update_when_disabled.md)
- [test_should_update_when_never_updated](../../../../../functions/src/config/update/test_should_update_when_never_updated.md)