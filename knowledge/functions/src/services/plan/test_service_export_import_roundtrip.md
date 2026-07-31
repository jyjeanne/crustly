---
type: Rust Function
title: test_service_export_import_roundtrip
resource: src/services/plan.rs#L576-L615
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/plan/setup_test_service
  - functions/src/services/plan/PlanService/export_to_json
  - functions/src/services/plan/PlanService/import_from_json
---

# Signature

`async fn test_service_export_import_roundtrip()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [export_to_json](../../../../functions/src/services/plan/PlanService/export_to_json.md)
- [import_from_json](../../../../functions/src/services/plan/PlanService/import_from_json.md)