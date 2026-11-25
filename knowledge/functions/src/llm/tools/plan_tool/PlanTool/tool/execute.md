---
type: Rust Method
title: execute
resource: src/llm/tools/plan_tool.rs#L325-L994
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/plan_tool/validate_plan_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/validate_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanDocument/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/validate_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_validation_warnings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/next_executable_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/execution_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_by_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/dependencies_satisfied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_by_order_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/add_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/complete_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/can_retry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/add_reflection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/record_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/complexity_stars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/progress_percentage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/completing_without_criteria_warns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [validate_plan_file_path](../../../../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [validate_string](../../../../../../../functions/src/llm/tools/plan_tool/validate_string.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [new](../../../../../../../functions/src/plan/PlanDocument/new.md)
- [new](../../../../../../../functions/src/plan/PlanTask/new.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
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
- [write](../../../../../../../functions/src/memory/project/write.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [complexity_stars](../../../../../../../functions/src/plan/PlanTask/complexity_stars.md)
- [progress_percentage](../../../../../../../functions/src/plan/PlanDocument/progress_percentage.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [acceptance_criteria_are_surfaced_at_start_and_completion](../../../../../../../functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion.md)
- [completing_without_criteria_warns](../../../../../../../functions/src/llm/tools/plan_tool/completing_without_criteria_warns.md)
- [sparse_plan_calls_execute_end_to_end](../../../../../../../functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end.md)