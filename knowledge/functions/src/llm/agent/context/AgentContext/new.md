---
type: Rust Method
title: new
resource: src/llm/agent/context.rs#L55-L66
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_fires_at_threshold
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/from_db_messages
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_context_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_add_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_would_exceed_limit
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_usage_percentage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/test_trim_to_fit
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/build_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/should_compact_fires_at_80_percent
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(session_id: Uuid, max_tokens: usize) -> Self`

# Called by

- [compact_keeping](../../../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [elide_tool_results_leaves_short_output_alone](../../../../../../functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_refuses_when_nothing_would_be_dropped](../../../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)
- [compaction_fires_at_threshold](../../../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [test_context_creation](../../../../../../functions/src/llm/agent/context/test_context_creation.md)
- [test_add_message](../../../../../../functions/src/llm/agent/context/test_add_message.md)
- [test_system_prompt](../../../../../../functions/src/llm/agent/context/test_system_prompt.md)
- [test_would_exceed_limit](../../../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)
- [test_usage_percentage](../../../../../../functions/src/llm/agent/context/test_usage_percentage.md)
- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [build_context](../../../../../../functions/tests/compaction_test/build_context.md)
- [should_compact_fires_at_80_percent](../../../../../../functions/tests/compaction_test/should_compact_fires_at_80_percent.md)