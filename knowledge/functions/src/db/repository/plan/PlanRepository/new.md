---
type: Rust Method
title: new
resource: src/db/repository/plan.rs#L26-L28
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/plan/setup_test_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [setup_test_db](../../../../../../functions/src/db/repository/plan/setup_test_db.md)
- [new](../../../../../../functions/src/services/plan/PlanService/new.md)