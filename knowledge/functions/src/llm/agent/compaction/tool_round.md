---
type: Rust Function
title: tool_round
resource: src/llm/agent/compaction.rs#L330-L349
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
  - target: functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_round(i: usize, output: &str) -> [Message; 2]`

# Called by

- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [elide_tool_results_leaves_short_output_alone](../../../../../functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)