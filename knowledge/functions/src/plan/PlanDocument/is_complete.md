---
type: Rust Method
title: is_complete
resource: src/plan/mod.rs#L173-L179
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn is_complete(&self) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)