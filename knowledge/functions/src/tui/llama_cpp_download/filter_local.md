---
type: Rust Function
title: filter_local
resource: src/tui/llama_cpp_download.rs#L418-L439
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_matches_display_name_even_when_filename_does_not
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_empty_query_returns_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local_no_match_returns_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn filter_local(models: &[LlamaCppModelSummary], query: &str) -> Vec<LlamaCppModelSummary>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [filter_local_matches_substring_case_insensitive](../../../../functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive.md)
- [filter_local_matches_display_name_even_when_filename_does_not](../../../../functions/src/tui/llama_cpp_download/filter_local_matches_display_name_even_when_filename_does_not.md)
- [filter_local_empty_query_returns_all](../../../../functions/src/tui/llama_cpp_download/filter_local_empty_query_returns_all.md)
- [filter_local_no_match_returns_empty](../../../../functions/src/tui/llama_cpp_download/filter_local_no_match_returns_empty.md)