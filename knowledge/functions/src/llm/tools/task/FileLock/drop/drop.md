---
type: Rust Method
title: drop
resource: src/llm/tools/task.rs#L153-L157
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/tilde_in_the_database_path_is_expanded_to_home
  - functions/src/llm/tools/powershell/PowerShellTool/tool/execute
  - functions/src/tui/app/App/execute_next_plan_task
  - functions/tests/integration_test/test_database_persistence
---

# Signature

`fn drop(&mut self)`

# Called by

- [tilde_in_the_database_path_is_expanded_to_home](../../../../../../../functions/src/db/tilde_in_the_database_path_is_expanded_to_home.md)
- [execute](../../../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [execute_next_plan_task](../../../../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [test_database_persistence](../../../../../../../functions/tests/integration_test/test_database_persistence.md)