---
type: Rust Function
title: filter_suggestions
resource: src/tui/ollama_download.rs#L53-L73
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/refresh_model_download_suggestions
  - functions/src/tui/ollama_download/filter_suggestions_empty_query_returns_all_deduped
  - functions/src/tui/ollama_download/filter_suggestions_matches_substring_case_insensitive
  - functions/src/tui/ollama_download/filter_suggestions_includes_ornith
---

# Signature

`pub fn filter_suggestions(query: &str, installed: &[String]) -> Vec<String>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [refresh_model_download_suggestions](../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [filter_suggestions_empty_query_returns_all_deduped](../../../../functions/src/tui/ollama_download/filter_suggestions_empty_query_returns_all_deduped.md)
- [filter_suggestions_matches_substring_case_insensitive](../../../../functions/src/tui/ollama_download/filter_suggestions_matches_substring_case_insensitive.md)
- [filter_suggestions_includes_ornith](../../../../functions/src/tui/ollama_download/filter_suggestions_includes_ornith.md)