---
type: Rust Function
title: test_json_export_import_integration
resource: tests/plan_mode_integration_test.rs#L336-L374
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/create_multi_task_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/export_to_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/import_from_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_json_export_import_integration()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [export_to_json](../../../functions/src/services/plan/PlanService/export_to_json.md)
- [import_from_json](../../../functions/src/services/plan/PlanService/import_from_json.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)