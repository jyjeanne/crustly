---
type: Rust Module
title: read
resource: src/llm/tools/read.rs#L1-L402
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-validate-file-path-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-io-asyncbufreadext-bufreader
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-future-join-all
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ReadTool](../../../../classes/src/llm/tools/read/ReadTool.md)
- [ReadInput](../../../../classes/src/llm/tools/read/ReadInput.md)
- [name](../../../../functions/src/llm/tools/read/ReadTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/read/ReadTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/read/ReadTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/read/ReadTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/read/ReadTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/read/ReadTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [read_with_buffer](../../../../functions/src/llm/tools/read/ReadTool/read_with_buffer.md)
- [test_read_file](../../../../functions/src/llm/tools/read/test_read_file.md)
- [test_read_file_line_range](../../../../functions/src/llm/tools/read/test_read_file_line_range.md)
- [test_read_nonexistent_file](../../../../functions/src/llm/tools/read/test_read_nonexistent_file.md)
- [test_read_file_accepts_file_path_alias](../../../../functions/src/llm/tools/read/test_read_file_accepts_file_path_alias.md)
- [test_read_tool_schema](../../../../functions/src/llm/tools/read/test_read_tool_schema.md)
- [test_five_concurrent_reads_no_deadlock](../../../../functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock.md)

# Imports

- `super::error::{validate_file_path, Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `tokio::fs`
- `tokio::io::{AsyncBufReadExt, BufReader}`
- `super::*`
- `std::io::Write`
- `tempfile::TempDir`
- `uuid::Uuid`
- `futures::future::join_all`
- `std::sync::Arc`

# Member of

- [crustly](../../../../packages/crustly.md)