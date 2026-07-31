---
type: Rust Method
title: import_from_json
resource: src/services/plan.rs#L141-L145
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/services/plan/test_service_import_from_json
  - functions/src/services/plan/test_service_export_import_roundtrip
  - functions/src/services/plan/test_service_json_import_nonexistent_file
  - functions/src/services/plan/test_service_json_import_invalid_json
  - functions/tests/plan_mode_integration_test/test_json_export_import_integration
---

# Signature

`pub async fn import_from_json(&self, file_path: &std::path::Path) -> Result<PlanDocument>`

# Calls

- [from_str](../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [test_service_import_from_json](../../../../../functions/src/services/plan/test_service_import_from_json.md)
- [test_service_export_import_roundtrip](../../../../../functions/src/services/plan/test_service_export_import_roundtrip.md)
- [test_service_json_import_nonexistent_file](../../../../../functions/src/services/plan/test_service_json_import_nonexistent_file.md)
- [test_service_json_import_invalid_json](../../../../../functions/src/services/plan/test_service_json_import_invalid_json.md)
- [test_json_export_import_integration](../../../../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)