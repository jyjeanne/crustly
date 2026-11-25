---
type: Rust Function
title: model
resource: src/tui/llama_cpp_download.rs#L445-L461
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_matches_display_name_even_when_filename_does_not
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_empty_query_returns_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_no_match_returns_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn model(name: &str) -> LlamaCppModelSummary`

# Called by

- [configure_agent](../../../../functions/src/cli/configure_agent.md)
- [filter_local_matches_display_name_even_when_filename_does_not](../../../../functions/src/tui/llama_cpp_download/filter_local_matches_display_name_even_when_filename_does_not.md)
- [filter_local_empty_query_returns_all](../../../../functions/src/tui/llama_cpp_download/filter_local_empty_query_returns_all.md)
- [filter_local_matches_substring_case_insensitive](../../../../functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive.md)
- [filter_local_no_match_returns_empty](../../../../functions/src/tui/llama_cpp_download/filter_local_no_match_returns_empty.md)