---
type: Rust Function
title: test_service_json_import_invalid_json
resource: src/services/plan.rs#L649-L657
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/import_from_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_service_json_import_invalid_json()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [write](../../../../functions/src/memory/project/write.md)
- [import_from_json](../../../../functions/src/services/plan/PlanService/import_from_json.md)