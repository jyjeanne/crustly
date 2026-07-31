---
type: Rust Method
title: validate_plan
resource: src/services/plan.rs#L148-L239
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`pub fn validate_plan(&self, plan: &PlanDocument) -> Vec<PlanValidationWarning>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)