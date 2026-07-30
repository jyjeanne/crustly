---
type: Rust Method
title: read_with_buffer
resource: src/llm/tools/read.rs#L161-L241
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/read/ReadTool/tool/execute
---

# Signature

`async fn read_with_buffer( &self, path: &std::path::Path, start_line: Option<usize>, line_count: Option<usize>, is_large_file: bool, ) -> Result<(String, usize, Option<String>)>`

# Calls

- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)