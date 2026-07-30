---
type: Rust Method
title: export_to_json
resource: src/services/plan.rs#L124-L137
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/plan/test_service_export_to_json
  - functions/src/services/plan/test_service_export_import_roundtrip
  - functions/src/services/plan/test_service_atomic_json_write
  - functions/src/tui/app/App/save_plan
  - functions/tests/plan_mode_integration_test/test_json_export_import_integration
---

# Signature

`pub async fn export_to_json( &self, plan: &PlanDocument, file_path: &std::path::Path, ) -> Result<()>`

# Called by

- [test_service_export_to_json](../../../../../functions/src/services/plan/test_service_export_to_json.md)
- [test_service_export_import_roundtrip](../../../../../functions/src/services/plan/test_service_export_import_roundtrip.md)
- [test_service_atomic_json_write](../../../../../functions/src/services/plan/test_service_atomic_json_write.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)
- [test_json_export_import_integration](../../../../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)