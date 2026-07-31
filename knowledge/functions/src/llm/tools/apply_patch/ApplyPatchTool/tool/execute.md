---
type: Rust Method
title: execute
resource: src/llm/tools/apply_patch.rs#L343-L518
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/apply_patch/parse_patch
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/error/validate_path_safety
  - functions/src/llm/tools/error/validate_file_path
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
  - functions/src/llm/tools/apply_patch/apply_hunks
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [parse_patch](../../../../../../../functions/src/llm/tools/apply_patch/parse_patch.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_path_safety](../../../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [validate_file_path](../../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [apply_hunks](../../../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)