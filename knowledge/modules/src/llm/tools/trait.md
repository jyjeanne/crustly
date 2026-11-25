---
type: Rust Module
title: trait
resource: src/llm/tools/trait.rs#L1-L344
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-file-read-cache-filereadcache
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-util-sync-cancellationtoken
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SubAgentTask](../../../../classes/src/llm/tools/trait/SubAgentTask.md)
- [SubAgentStatus](../../../../classes/src/llm/tools/trait/SubAgentStatus.md)
- [is_finished](../../../../functions/src/llm/tools/trait/SubAgentStatus/is_finished.md)
- [SubAgentReport](../../../../classes/src/llm/tools/trait/SubAgentReport.md)
- [SubAgentLauncher](../../../../interfaces/src/llm/tools/trait/SubAgentLauncher.md)
- [ToolExecutionContext](../../../../classes/src/llm/tools/trait/ToolExecutionContext.md)
- [new](../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_cancel](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_cancel.md)
- [with_working_directory](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [with_auto_approve](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)
- [with_timeout](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_timeout.md)
- [with_read_only_mode](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [with_sub_agent_launcher](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher.md)
- [with_file_read_cache](../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_file_read_cache.md)
- [ToolResult](../../../../classes/src/llm/tools/trait/ToolResult.md)
- [success](../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [error](../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [with_metadata](../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [ToolCapability](../../../../classes/src/llm/tools/trait/ToolCapability.md)
- [Tool](../../../../interfaces/src/llm/tools/trait/Tool.md)
- [requires_approval](../../../../functions/src/llm/tools/trait/Tool/requires_approval.md)
- [requires_approval_for](../../../../functions/src/llm/tools/trait/Tool/requires_approval_for.md)
- [approval_preview](../../../../functions/src/llm/tools/trait/Tool/approval_preview.md)
- [validate_input](../../../../functions/src/llm/tools/trait/Tool/validate_input.md)
- [test_execution_context](../../../../functions/src/llm/tools/trait/test_execution_context.md)
- [test_tool_result_success](../../../../functions/src/llm/tools/trait/test_tool_result_success.md)
- [test_tool_result_error](../../../../functions/src/llm/tools/trait/test_tool_result_error.md)

# Imports

- `super::error::Result`
- `super::file_read_cache::FileReadCache`
- `async_trait::async_trait`
- `serde::Serialize`
- `serde_json::Value`
- `std::collections::HashMap`
- `std::sync::Arc`
- `tokio_util::sync::CancellationToken`
- `uuid::Uuid`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)