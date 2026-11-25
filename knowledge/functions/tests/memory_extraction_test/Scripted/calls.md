---
type: Rust Method
title: calls
resource: tests/memory_extraction_test.rs#L36-L38
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calls(&self) -> usize`

# Called by

- [the_background_trigger_runs_once_per_session_at_a_time](../../../../functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time.md)
- [extraction_is_off_by_default_and_calls_nothing](../../../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [a_run_saves_what_the_model_proposes_and_moves_the_watermark](../../../../functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark.md)
- [below_the_every_n_count_nothing_runs_unless_forced](../../../../functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced.md)
- [a_range_the_provider_always_fails_on_is_skipped_after_three_tries](../../../../functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries.md)
- [a_long_backlog_is_read_over_several_runs](../../../../functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs.md)