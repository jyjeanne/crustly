---
type: Rust Method
title: execution_summary
resource: src/plan/mod.rs#L309-L337
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn execution_summary(&self) -> ExecutionSummary`

# Calls

- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)