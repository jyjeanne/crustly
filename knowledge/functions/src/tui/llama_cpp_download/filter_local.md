---
type: Rust Function
title: filter_local
resource: src/tui/llama_cpp_download.rs#L269-L284
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive
---

# Signature

`pub fn filter_local(models: &[LlamaCppModelSummary], query: &str) -> Vec<LlamaCppModelSummary>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [filter_local_matches_substring_case_insensitive](../../../../functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive.md)