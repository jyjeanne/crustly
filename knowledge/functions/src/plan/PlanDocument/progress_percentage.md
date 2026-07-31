---
type: Rust Method
title: progress_percentage
resource: src/plan/mod.rs#L164-L170
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanDocument/count_by_status
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/services/plan/PlanService/get_statistics
---

# Signature

`pub fn progress_percentage(&self) -> f32`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [count_by_status](../../../../functions/src/plan/PlanDocument/count_by_status.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [get_statistics](../../../../functions/src/services/plan/PlanService/get_statistics.md)