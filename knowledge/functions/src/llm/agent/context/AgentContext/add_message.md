---
type: Rust Method
title: add_message
resource: src/llm/agent/context.rs#L68-L73
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/estimate_message_tokens
  called_by:
  - functions/src/llm/agent/compaction/compaction_fires_at_threshold
  - functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged
  - functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
  - functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
  - functions/src/llm/agent/context/AgentContext/from_db_messages
  - functions/src/llm/agent/context/test_add_message
  - functions/src/llm/agent/context/test_would_exceed_limit
  - functions/src/llm/agent/context/test_usage_percentage
  - functions/src/llm/agent/context/test_trim_to_fit
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/tests/compaction_test/build_context
---

# Signature

`pub fn add_message(&mut self, message: Message)`

# Calls

- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)

# Called by

- [compaction_fires_at_threshold](../../../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [test_add_message](../../../../../../functions/src/llm/agent/context/test_add_message.md)
- [test_would_exceed_limit](../../../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)
- [test_usage_percentage](../../../../../../functions/src/llm/agent/context/test_usage_percentage.md)
- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [build_context](../../../../../../functions/tests/compaction_test/build_context.md)