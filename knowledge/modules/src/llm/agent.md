---
type: Rust Module
title: agent
resource: src/llm/agent/mod.rs#L1-L22
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-budget-contextbudget-contextusage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-context-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-error-agenterror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-service-agentresponse-agentservice-agentstreamresponse-approvalcallback-fallback-toolapprovalinfo
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use budget::{ContextBudget, ContextUsage}`
- `pub use context::AgentContext`
- `pub use error::{AgentError, Result}`
- `pub use service::{
    AgentResponse, AgentService, AgentStreamResponse, ApprovalCallback, Fallback, ToolApprovalInfo,
}`

# Member of

- [crustly](../../../packages/crustly.md)