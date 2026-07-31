---
type: Rust Method
title: execute
resource: src/llm/tools/edit.rs#L212-L414
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/edit/normalize_input
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/error/validate_file_path
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [normalize_input](../../../../../../../functions/src/llm/tools/edit/normalize_input.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_file_path](../../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)