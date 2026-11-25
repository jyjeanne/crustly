---
type: Rust Function
title: tool_results_are_paired
resource: src/llm/agent/compaction.rs#L352-L368
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_results_are_paired(messages: &[Message]) -> bool`

# Called by

- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)