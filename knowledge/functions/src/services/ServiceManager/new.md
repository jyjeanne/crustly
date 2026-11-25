---
type: Rust Method
title: new
resource: src/services/mod.rs#L51-L61
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/test_service_manager_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: Pool) -> Self`

# Calls

- [new](../../../../functions/src/services/ServiceContext/new.md)
- [new](../../../../functions/src/services/session/SessionService/new.md)
- [new](../../../../functions/src/services/message/MessageService/new.md)
- [new](../../../../functions/src/services/file/FileService/new.md)
- [new](../../../../functions/src/services/plan/PlanService/new.md)

# Called by

- [test_service_manager_creation](../../../../functions/src/services/test_service_manager_creation.md)