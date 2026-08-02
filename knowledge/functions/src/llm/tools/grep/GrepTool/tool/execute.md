---
type: Rust Method
title: execute
resource: src/llm/tools/grep.rs#L141-L234
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/grep/GrepTool/search_file
  - functions/src/llm/tools/grep/collect_searchable_files
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [search_file](../../../../../../../functions/src/llm/tools/grep/GrepTool/search_file.md)
- [collect_searchable_files](../../../../../../../functions/src/llm/tools/grep/collect_searchable_files.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)