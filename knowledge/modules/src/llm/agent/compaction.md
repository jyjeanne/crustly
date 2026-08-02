---
type: Rust Module
title: compaction
resource: src/llm/agent/compaction.rs#L1-L401
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-llm-agent-context-agentcontext
  - external/anyhow-result
  - external/chrono-datetime-utc
  - external/uuid-uuid
  - external/crate-llm-provider-types-contentblock-message-role
  - external/crate-llm-provider-types-contentblock
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [CompactionRecord](../../../../classes/src/llm/agent/compaction/CompactionRecord.md)
- [compact](../../../../functions/src/llm/agent/compaction/compact.md)
- [message_has_tool_result](../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)
- [summarise_turns](../../../../functions/src/llm/agent/compaction/summarise_turns.md)
- [summarise_turns_truncates_multibyte_text_without_panicking](../../../../functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking.md)
- [compaction_fires_at_threshold](../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)

# Imports

- `crate::llm::agent::context::AgentContext`
- `anyhow::Result`
- `chrono::{DateTime, Utc}`
- `uuid::Uuid`
- `crate::llm::provider::types::{ContentBlock, Message, Role}`
- `crate::llm::provider::types::ContentBlock`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)