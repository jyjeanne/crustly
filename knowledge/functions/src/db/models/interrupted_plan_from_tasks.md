---
type: Rust Function
title: interrupted_plan_from_tasks
resource: src/db/models.rs#L195-L216
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTask/exec_status
  - functions/src/db/models/PlanTaskStatus/is_incomplete
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/tests/plan_crash_recovery_test/interrupted_plan_none_when_all_done
  - functions/tests/plan_crash_recovery_test/interrupted_plan_resumes_at_lowest_incomplete
---

# Signature

`pub fn interrupted_plan_from_tasks( plan_id: Uuid, tasks: &[PlanTask], ) -> Option<crate::plan::InterruptedPlan>`

# Calls

- [exec_status](../../../../functions/src/db/models/PlanTask/exec_status.md)
- [is_incomplete](../../../../functions/src/db/models/PlanTaskStatus/is_incomplete.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [interrupted_plan_none_when_all_done](../../../../functions/tests/plan_crash_recovery_test/interrupted_plan_none_when_all_done.md)
- [interrupted_plan_resumes_at_lowest_incomplete](../../../../functions/tests/plan_crash_recovery_test/interrupted_plan_resumes_at_lowest_incomplete.md)