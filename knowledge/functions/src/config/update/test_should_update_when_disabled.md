---
type: Rust Function
title: test_should_update_when_disabled
resource: src/config/update.rs#L228-L242
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/ProviderUpdater/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/update/ProviderUpdater/should_update
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_should_update_when_disabled()`

# Calls

- [new](../../../../functions/src/config/crabrace/CrabraceIntegration/new.md)
- [new](../../../../functions/src/config/update/ProviderUpdater/new.md)
- [should_update](../../../../functions/src/config/update/ProviderUpdater/should_update.md)