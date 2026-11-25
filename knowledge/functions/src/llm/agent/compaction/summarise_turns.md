---
type: Rust Function
title: summarise_turns
resource: src/llm/agent/compaction.rs#L214-L247
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/summarise_turns_records_tool_calls_and_results
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn summarise_turns(messages: &[(usize, &Message)]) -> String`

# Called by

- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [summarise_turns_truncates_multibyte_text_without_panicking](../../../../../functions/src/llm/agent/compaction/summarise_turns_truncates_multibyte_text_without_panicking.md)
- [summarise_turns_records_tool_calls_and_results](../../../../../functions/src/llm/agent/compaction/summarise_turns_records_tool_calls_and_results.md)