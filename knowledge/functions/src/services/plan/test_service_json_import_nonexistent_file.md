---
type: Rust Function
title: test_service_json_import_nonexistent_file
resource: src/services/plan.rs#L639-L646
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
  - target: functions/src/services/plan/PlanService/import_from_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_service_json_import_nonexistent_file()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [import_from_json](../../../../functions/src/services/plan/PlanService/import_from_json.md)