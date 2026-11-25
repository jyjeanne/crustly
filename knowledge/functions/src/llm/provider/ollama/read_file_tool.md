---
type: Rust Function
title: read_file_tool
resource: src/llm/provider/ollama.rs#L1719-L1728
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_file_tool() -> Tool`

# Called by

- [printed_tool_call_is_withheld_then_recovered](../../../../../functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered.md)
- [withheld_text_that_is_not_a_tool_call_is_flushed](../../../../../functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed.md)
- [json_after_text_is_streamed_as_text_and_never_run](../../../../../functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run.md)