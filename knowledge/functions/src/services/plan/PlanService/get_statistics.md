---
type: Rust Method
title: get_statistics
resource: src/services/plan.rs#L270-L326
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanDocument/progress_percentage
---

# Signature

`pub async fn get_statistics(&self, session_id: Uuid) -> Result<PlanStatistics>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [progress_percentage](../../../../../functions/src/plan/PlanDocument/progress_percentage.md)