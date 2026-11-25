---
type: Rust Module
title: tool_round
resource: src/llm/agent/service/tool_round.rs#L1-L386
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-has-mutating-capability-is-parallelizable-plan-completion-rejection-agentservice-requesttools-toolapprovalinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-middleware-toolcallrecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-tool-profile-toolselection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentblock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-cache-cachekey
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-schema-check-check-arguments-describe-arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-toolcapability-toolerror-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-future-join-all
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-util-sync-cancellationtoken
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ToolRound](../../../../../classes/src/llm/agent/service/tool_round/ToolRound.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [refuse](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/refuse.md)
- [refused](../../../../../functions/src/llm/agent/service/tool_round/refused.md)
- [completed](../../../../../functions/src/llm/agent/service/tool_round/completed.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)

# Imports

- `super::{
    has_mutating_capability, is_parallelizable, plan_completion_rejection, AgentService,
    RequestTools, ToolApprovalInfo,
}`
- `crate::llm::agent::middleware::ToolCallRecord`
- `crate::llm::agent::tool_profile::ToolSelection`
- `crate::llm::provider::ContentBlock`
- `crate::llm::tools::cache::CacheKey`
- `crate::llm::tools::schema_check::{check_arguments, describe_arguments}`
- `crate::llm::tools::{ToolCapability, ToolError, ToolExecutionContext, ToolResult}`
- `futures::future::join_all`
- `serde_json::Value`
- `std::sync::Arc`
- `tokio_util::sync::CancellationToken`

# Member of

- [crustly](../../../../../packages/crustly.md)