---
type: Rust Method
title: get_validation_warnings
resource: src/plan/mod.rs#L355-L412
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
---

# Signature

`pub fn get_validation_warnings(&self) -> Vec<String>`

# Calls

- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)