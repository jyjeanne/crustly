---
type: Rust Function
title: apply_hunks
resource: src/llm/tools/apply_patch.rs#L235-L285
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/apply_patch/find_subsequence
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/apply_patch/apply_hunks_replaces_matched_context
  - functions/src/llm/tools/apply_patch/apply_hunks_second_hunk_searches_after_first
  - functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found
---

# Signature

`fn apply_hunks(original: &str, hunks: &[Hunk]) -> std::result::Result<String, String>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [find_subsequence](../../../../../functions/src/llm/tools/apply_patch/find_subsequence.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [apply_hunks_replaces_matched_context](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_replaces_matched_context.md)
- [apply_hunks_second_hunk_searches_after_first](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_second_hunk_searches_after_first.md)
- [apply_hunks_errors_when_context_not_found](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found.md)