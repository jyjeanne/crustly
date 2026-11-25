---
type: Rust Method
title: read_with_buffer
resource: src/llm/tools/read.rs#L161-L241
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn read_with_buffer( &self, path: &std::path::Path, start_line: Option<usize>, line_count: Option<usize>, is_large_file: bool, ) -> Result<(String, usize, Option<String>)>`

# Calls

- [open](../../../../../../functions/tests/memory_acceptance_test/open.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)