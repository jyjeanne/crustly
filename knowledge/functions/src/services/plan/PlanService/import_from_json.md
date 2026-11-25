---
type: Rust Method
title: import_from_json
resource: src/services/plan.rs#L141-L145
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/services/plan/test_service_import_from_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_export_import_roundtrip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_json_import_nonexistent_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_json_import_invalid_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_json_export_import_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn import_from_json(&self, file_path: &std::path::Path) -> Result<PlanDocument>`

# Called by

- [test_service_import_from_json](../../../../../functions/src/services/plan/test_service_import_from_json.md)
- [test_service_export_import_roundtrip](../../../../../functions/src/services/plan/test_service_export_import_roundtrip.md)
- [test_service_json_import_nonexistent_file](../../../../../functions/src/services/plan/test_service_json_import_nonexistent_file.md)
- [test_service_json_import_invalid_json](../../../../../functions/src/services/plan/test_service_json_import_invalid_json.md)
- [test_json_export_import_integration](../../../../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)