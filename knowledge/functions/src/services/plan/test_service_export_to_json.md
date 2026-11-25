---
type: Rust Function
title: test_service_export_to_json
resource: src/services/plan.rs#L530-L551
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/export_to_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_service_export_to_json()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [export_to_json](../../../../functions/src/services/plan/PlanService/export_to_json.md)