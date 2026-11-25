---
type: Rust Module
title: run
resource: src/llm/agent/service/run.rs#L1-L746
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-tool-round-tooluse
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-agentservice-requesttools-max-overflow-retries
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-budget-contextbudget
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-context-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-error-agenterror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-middleware-decision-pipeline-runstate-toolcallrecord-toolcallview
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-tool-profile-toolselection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentblock-llmresponse-message-provider-providererror-role
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-toolexecutioncontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-util-sync-cancellationtoken
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AgentRun](../../../../../classes/src/llm/agent/service/run/AgentRun.md)
- [RunOutcome](../../../../../classes/src/llm/agent/service/run/RunOutcome.md)
- [Totals](../../../../../classes/src/llm/agent/service/run/Totals.md)
- [run](../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [path_memory_text](../../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [outcome](../../../../../functions/src/llm/agent/service/run/AgentRun/outcome.md)
- [add_usage](../../../../../functions/src/llm/agent/service/run/AgentRun/add_usage.md)
- [check_cancelled](../../../../../functions/src/llm/agent/service/run/AgentRun/check_cancelled.md)
- [call_model](../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [compact_deeper](../../../../../functions/src/llm/agent/service/run/AgentRun/compact_deeper.md)
- [fall_back](../../../../../functions/src/llm/agent/service/run/AgentRun/fall_back.md)
- [finish](../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)
- [log_blocks](../../../../../functions/src/llm/agent/service/run/log_blocks.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [notes_for_src](../../../../../functions/src/llm/agent/service/run/notes_for_src.md)
- [an_expired_path_memory_is_not_sent](../../../../../functions/src/llm/agent/service/run/an_expired_path_memory_is_not_sent.md)
- [a_path_memory_cannot_close_the_notes_block](../../../../../functions/src/llm/agent/service/run/a_path_memory_cannot_close_the_notes_block.md)

# Imports

- `super::tool_round::ToolUse`
- `super::{AgentService, RequestTools, MAX_OVERFLOW_RETRIES}`
- `crate::llm::agent::budget::ContextBudget`
- `crate::llm::agent::context::AgentContext`
- `crate::llm::agent::error::{AgentError, Result}`
- `crate::llm::agent::middleware::{Decision, Pipeline, RunState, ToolCallRecord, ToolCallView}`
- `crate::llm::agent::tool_profile::ToolSelection`
- `crate::llm::provider::{ContentBlock, LLMResponse, Message, Provider, ProviderError, Role}`
- `crate::llm::tools::ToolExecutionContext`
- `std::sync::Arc`
- `std::time::Instant`
- `tokio::sync::mpsc`
- `tokio_util::sync::CancellationToken`
- `super::*`
- `crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`

# Member of

- [crustly](../../../../../packages/crustly.md)