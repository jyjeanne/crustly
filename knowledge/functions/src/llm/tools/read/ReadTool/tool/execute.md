---
type: Rust Method
title: execute
resource: src/llm/tools/read.rs#L92-L156
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/error/validate_file_path
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/read/ReadTool/read_with_buffer
  - functions/src/llm/tools/file_read_cache/FileReadCache/record
  - functions/src/llm/tools/file_read_cache/FileFingerprint/of
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_file_path](../../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [read_with_buffer](../../../../../../../functions/src/llm/tools/read/ReadTool/read_with_buffer.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)