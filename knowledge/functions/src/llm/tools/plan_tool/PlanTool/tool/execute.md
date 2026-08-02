---
type: Rust Method
title: execute
resource: src/llm/tools/plan_tool.rs#L325-L994
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/plan_tool/validate_plan_file_path
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/llm/tools/plan_tool/validate_string
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanDocument/add_task
  - functions/src/plan/PlanDocument/validate_dependencies
  - functions/src/plan/PlanDocument/get_validation_warnings
  - functions/src/plan/PlanDocument/next_executable_task
  - functions/src/plan/PlanDocument/execution_summary
  - functions/src/plan/PlanDocument/get_task_by_order
  - functions/src/plan/PlanDocument/dependencies_satisfied
  - functions/src/plan/PlanDocument/get_task_by_order_mut
  - functions/src/plan/PlanTask/add_artifact
  - functions/src/plan/PlanTask/complete_execution
  - functions/src/plan/PlanTask/can_retry
  - functions/src/plan/PlanDocument/is_complete
  - functions/src/plan/PlanTask/add_reflection
  - functions/src/plan/PlanTask/record_tool_call
  - functions/src/plan/PlanTask/skip
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [validate_plan_file_path](../../../../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [from_str](../../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [validate_string](../../../../../../../functions/src/llm/tools/plan_tool/validate_string.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [add_task](../../../../../../../functions/src/plan/PlanDocument/add_task.md)
- [validate_dependencies](../../../../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [get_validation_warnings](../../../../../../../functions/src/plan/PlanDocument/get_validation_warnings.md)
- [next_executable_task](../../../../../../../functions/src/plan/PlanDocument/next_executable_task.md)
- [execution_summary](../../../../../../../functions/src/plan/PlanDocument/execution_summary.md)
- [get_task_by_order](../../../../../../../functions/src/plan/PlanDocument/get_task_by_order.md)
- [dependencies_satisfied](../../../../../../../functions/src/plan/PlanDocument/dependencies_satisfied.md)
- [get_task_by_order_mut](../../../../../../../functions/src/plan/PlanDocument/get_task_by_order_mut.md)
- [add_artifact](../../../../../../../functions/src/plan/PlanTask/add_artifact.md)
- [complete_execution](../../../../../../../functions/src/plan/PlanTask/complete_execution.md)
- [can_retry](../../../../../../../functions/src/plan/PlanTask/can_retry.md)
- [is_complete](../../../../../../../functions/src/plan/PlanDocument/is_complete.md)
- [add_reflection](../../../../../../../functions/src/plan/PlanTask/add_reflection.md)
- [record_tool_call](../../../../../../../functions/src/plan/PlanTask/record_tool_call.md)
- [skip](../../../../../../../functions/src/plan/PlanTask/skip.md)