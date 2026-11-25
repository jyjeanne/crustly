---
type: Rust Method
title: new
resource: src/services/file.rs#L20-L22
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/services/file/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(context: ServiceContext) -> Self`

# Called by

- [create_test_service](../../../../../functions/src/services/file/create_test_service.md)
- [new](../../../../../functions/src/services/ServiceManager/new.md)