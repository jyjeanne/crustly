---
type: Rust Function
title: test_json_export_import_integration
resource: tests/plan_mode_integration_test.rs#L336-L374
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/tests/plan_mode_integration_test/setup_test_env
  - functions/tests/plan_mode_integration_test/create_multi_task_plan
  - functions/src/services/plan/PlanService/export_to_json
  - functions/src/services/plan/PlanService/import_from_json
---

# Signature

`async fn test_json_export_import_integration()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [export_to_json](../../../functions/src/services/plan/PlanService/export_to_json.md)
- [import_from_json](../../../functions/src/services/plan/PlanService/import_from_json.md)