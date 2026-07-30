---
type: Rust Method
title: execute
resource: src/llm/tools/write.rs#L87-L222
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/error/validate_path_safety
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
  - functions/src/llm/tools/trait/ToolResult/with_metadata
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_path_safety](../../../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)