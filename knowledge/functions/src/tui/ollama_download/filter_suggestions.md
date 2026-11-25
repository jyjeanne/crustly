---
type: Rust Function
title: filter_suggestions
resource: src/tui/ollama_download.rs#L53-L73
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/refresh_model_download_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/filter_suggestions_empty_query_returns_all_deduped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/filter_suggestions_matches_substring_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/filter_suggestions_includes_ornith
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn filter_suggestions(query: &str, installed: &[String]) -> Vec<String>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [refresh_model_download_suggestions](../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [filter_suggestions_empty_query_returns_all_deduped](../../../../functions/src/tui/ollama_download/filter_suggestions_empty_query_returns_all_deduped.md)
- [filter_suggestions_matches_substring_case_insensitive](../../../../functions/src/tui/ollama_download/filter_suggestions_matches_substring_case_insensitive.md)
- [filter_suggestions_includes_ornith](../../../../functions/src/tui/ollama_download/filter_suggestions_includes_ornith.md)