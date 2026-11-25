---
type: Rust Module
title: compaction
resource: src/llm/agent/compaction.rs#L1-L704
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crate-llm-agent-context-token-count-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-types-contentblock-message-role
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-utils-truncate-at-char-boundary-as-cut
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-context-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CompactionRecord](../../../../classes/src/llm/agent/compaction/CompactionRecord.md)
- [compact](../../../../functions/src/llm/agent/compaction/compact.md)
- [compact_keeping](../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [elide_tool_results](../../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [CompactionPlan](../../../../classes/src/llm/agent/compaction/CompactionPlan.md)
- [plan_compaction](../../../../functions/src/llm/agent/compaction/plan_compaction.md)
- [message_has_tool_result](../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)
- [is_plain_user_message](../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)
- [summarise_turns](../../../../functions/src/llm/agent/compaction/summarise_turns.md)
- [merge_summaries](../../../../functions/src/llm/agent/compaction/merge_summaries.md)
- [summarise_turns_truncates_multibyte_text_without_panicking](../../../../functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking.md)
- [summarise_turns_records_tool_calls_and_results](../../../../functions/src/llm/agent/compaction/summarise_turns_records_tool_calls_and_results.md)
- [merge_summaries_drops_the_oldest_lines_past_the_cap](../../../../functions/src/llm/agent/compaction/merge_summaries_drops_the_oldest_lines_past_the_cap.md)
- [tool_round](../../../../functions/src/llm/agent/compaction/tool_round.md)
- [tool_results_are_paired](../../../../functions/src/llm/agent/compaction/tool_results_are_paired.md)
- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [elide_tool_results_leaves_short_output_alone](../../../../functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone.md)
- [pool_with_compaction_table](../../../../functions/src/llm/agent/compaction/pool_with_compaction_table.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_refuses_when_nothing_would_be_dropped](../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)
- [compaction_fires_at_threshold](../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)

# Imports

- `crate::llm::agent::context::{token_count, AgentContext}`
- `crate::llm::provider::types::{ContentBlock, Message, Role}`
- `anyhow::Result`
- `chrono::{DateTime, Utc}`
- `uuid::Uuid`
- `crate::utils::truncate_at_char_boundary as cut`
- `super::*`
- `crate::llm::agent::context::AgentContext`

# Member of

- [crustly](../../../../packages/crustly.md)