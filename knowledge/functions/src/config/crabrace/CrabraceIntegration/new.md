---
type: Rust Method
title: new
resource: src/config/crabrace.rs#L64-L68
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/crabrace/test_create_integration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/crabrace/test_health_check
    resolved_by: tree-sitter-receiver
    confidence: semantic
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

`pub fn new(config: CrabraceConfig) -> Result<Self>`

# Called by

- [test_create_integration](../../../../../functions/src/config/crabrace/test_create_integration.md)
- [test_health_check](../../../../../functions/src/config/crabrace/test_health_check.md)
- [update_once](../../../../../functions/src/config/update/ProviderUpdater/update_once.md)
- [test_should_update_when_disabled](../../../../../functions/src/config/update/test_should_update_when_disabled.md)
- [test_should_update_when_never_updated](../../../../../functions/src/config/update/test_should_update_when_never_updated.md)