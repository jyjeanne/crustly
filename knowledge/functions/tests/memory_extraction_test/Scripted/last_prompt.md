---
type: Rust Method
title: last_prompt
resource: tests/memory_extraction_test.rs#L40-L47
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn last_prompt(&self) -> String`

# Called by

- [the_model_is_never_shown_tool_output_or_secrets](../../../../functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets.md)
- [a_session_end_run_over_a_long_backlog_reads_the_newest_turns](../../../../functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns.md)