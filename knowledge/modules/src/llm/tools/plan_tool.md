---
type: Rust Module
title: plan_tool
resource: src/llm/tools/plan_tool.rs#L1-L1168
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/crate-plan-plandocument-planstatus-plantask-tasktype-toolcall-as-plantoolcall
  - external/async-trait-async-trait
  - external/chrono-utc
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-path
  - external/super
  - external/tempfile-tempdir
  member_of:
  - packages/crustly
---

# Contains

- [PlanTool](../../../../classes/src/llm/tools/plan_tool/PlanTool.md)
- [PlanOperation](../../../../classes/src/llm/tools/plan_tool/PlanOperation.md)
- [default_complexity](../../../../functions/src/llm/tools/plan_tool/default_complexity.md)
- [default_task_type](../../../../functions/src/llm/tools/plan_tool/default_task_type.md)
- [validate_plan_file_path](../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [validate_string](../../../../functions/src/llm/tools/plan_tool/validate_string.md)
- [name](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [title_only_create_and_add_task_are_valid](../../../../functions/src/llm/tools/plan_tool/title_only_create_and_add_task_are_valid.md)
- [acceptance_criteria_are_surfaced_at_start_and_completion](../../../../functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion.md)
- [completing_without_criteria_warns](../../../../functions/src/llm/tools/plan_tool/completing_without_criteria_warns.md)
- [sparse_plan_calls_execute_end_to_end](../../../../functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `crate::plan::{PlanDocument, PlanStatus, PlanTask, TaskType, ToolCall as PlanToolCall}`
- `async_trait::async_trait`
- `chrono::Utc`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::Path`
- `super::*`
- `tempfile::TempDir`

# Member of

- [crustly](../../../../packages/crustly.md)