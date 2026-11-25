---
type: Rust Module
title: agent
resource: src/llm/tools/agent.rs#L1-L459
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-subagentreport-subagentstatus-subagenttask-tool-toolcapability-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AgentTool](../../../../classes/src/llm/tools/agent/AgentTool.md)
- [AgentInput](../../../../classes/src/llm/tools/agent/AgentInput.md)
- [name](../../../../functions/src/llm/tools/agent/AgentTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/agent/AgentTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/agent/AgentTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/agent/AgentTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/agent/AgentTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/agent/AgentTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [action_of](../../../../functions/src/llm/tools/agent/action_of.md)
- [parse_id](../../../../functions/src/llm/tools/agent/parse_id.md)
- [profile_for_type](../../../../functions/src/llm/tools/agent/profile_for_type.md)
- [status_word](../../../../functions/src/llm/tools/agent/status_word.md)
- [report_result](../../../../functions/src/llm/tools/agent/report_result.md)
- [is_blank](../../../../functions/src/llm/tools/agent/is_blank.md)
- [test_validate_empty_description](../../../../functions/src/llm/tools/agent/test_validate_empty_description.md)
- [test_validate_empty_prompt](../../../../functions/src/llm/tools/agent/test_validate_empty_prompt.md)
- [test_validate_valid_input](../../../../functions/src/llm/tools/agent/test_validate_valid_input.md)
- [validate_checks_each_action_s_own_fields](../../../../functions/src/llm/tools/agent/validate_checks_each_action_s_own_fields.md)
- [older_type_hints_map_to_profiles](../../../../functions/src/llm/tools/agent/older_type_hints_map_to_profiles.md)
- [FakeLauncher](../../../../classes/src/llm/tools/agent/FakeLauncher.md)
- [run](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/run.md)
- [start](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/start.md)
- [report](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/report.md)
- [cancel](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/cancel.md)
- [list](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/list.md)
- [profiles](../../../../functions/src/llm/tools/agent/FakeLauncher/crate-llm-tools-subagentlauncher/profiles.md)
- [run_waits_for_the_answer_and_returns_it](../../../../functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it.md)
- [without_a_launcher_the_tool_says_so](../../../../functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so.md)
- [a_failed_report_is_a_failed_tool_call](../../../../functions/src/llm/tools/agent/a_failed_report_is_a_failed_tool_call.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{
    SubAgentReport, SubAgentStatus, SubAgentTask, Tool, ToolCapability, ToolExecutionContext,
    ToolResult,
}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `uuid::Uuid`
- `super::*`
- `std::sync::Mutex`

# Member of

- [crustly](../../../../packages/crustly.md)