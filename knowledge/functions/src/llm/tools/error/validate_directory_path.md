---
type: Rust Function
title: validate_directory_path
resource: src/llm/tools/error.rs#L154-L183
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/error/validate_path_safety
---

# Signature

`pub fn validate_directory_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>`

# Calls

- [validate_path_safety](../../../../../functions/src/llm/tools/error/validate_path_safety.md)