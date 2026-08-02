---
type: Rust Function
title: test_service_export_to_json
resource: src/services/plan.rs#L530-L551
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/plan/setup_test_service
  - functions/src/services/plan/PlanService/export_to_json
  - functions/src/config/secrets/SecretString/from_str
---

# Signature

`async fn test_service_export_to_json()`

# Calls

- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [export_to_json](../../../../functions/src/services/plan/PlanService/export_to_json.md)
- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)