---
type: Rust Function
title: test_health_check
resource: src/config/crabrace.rs#L135-L143
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/crabrace/CrabraceIntegration/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/crabrace/CrabraceIntegration/health_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_health_check()`

# Calls

- [default](../../../../functions/src/config/crabrace/CrabraceConfig/default/default.md)
- [new](../../../../functions/src/config/crabrace/CrabraceIntegration/new.md)
- [health_check](../../../../functions/src/config/crabrace/CrabraceIntegration/health_check.md)