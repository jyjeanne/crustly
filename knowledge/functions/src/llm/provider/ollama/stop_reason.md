---
type: Rust Function
title: stop_reason
resource: src/llm/provider/ollama.rs#L1712-L1717
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stop_reason(events: &[StreamEvent]) -> Option<StopReason>`

# Called by

- [printed_tool_call_is_withheld_then_recovered](../../../../../functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered.md)
- [withheld_text_that_is_not_a_tool_call_is_flushed](../../../../../functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [json_after_text_is_streamed_as_text_and_never_run](../../../../../functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run.md)