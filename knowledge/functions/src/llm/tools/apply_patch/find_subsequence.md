---
type: Rust Function
title: find_subsequence
resource: src/llm/tools/apply_patch.rs#L220-L229
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/apply_patch/apply_hunks
---

# Signature

`fn find_subsequence(haystack: &[String], needle: &[&str], start: usize) -> Option<usize>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [apply_hunks](../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)