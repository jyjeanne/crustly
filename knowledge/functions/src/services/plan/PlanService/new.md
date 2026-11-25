---
type: Rust Method
title: new
resource: src/services/plan.rs#L50-L55
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/PlanTaskRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/ServiceManager/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(context: ServiceContext) -> Self`

# Calls

- [new](../../../../../functions/src/db/repository/plan/PlanRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [new](../../../../../functions/src/db/repository/plan/PlanTaskRepository/new.md)

# Called by

- [new](../../../../../functions/src/services/ServiceManager/new.md)
- [setup_test_service](../../../../../functions/src/services/plan/setup_test_service.md)
- [new](../../../../../functions/src/tui/app/App/new.md)
- [setup_test_env](../../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)