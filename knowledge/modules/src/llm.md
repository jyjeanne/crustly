---
type: Rust Module
title: llm
resource: src/llm/mod.rs#L1-L18
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-provider-anthropicprovider-contentblock-llmrequest-llmresponse-message-provider-providererror-providerstream-role-stopreason-streamevent-tokenusage-tool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-agent-agentcontext-agenterror-agentservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-tools-toolerror-toolregistry-toolresult
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use provider::{
    AnthropicProvider, ContentBlock, LLMRequest, LLMResponse, Message, Provider, ProviderError,
    ProviderStream, Role, StopReason, StreamEvent, TokenUsage, Tool,
}`
- `pub use agent::{AgentContext, AgentError, AgentService}`
- `pub use tools::{ToolError, ToolRegistry, ToolResult}`

# Member of

- [crustly](../../packages/crustly.md)