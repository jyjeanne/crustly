---
type: Rust Module
title: read
resource: src/llm/tools/read.rs#L1-L402
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-validate-file-path-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/tokio-fs
  - external/tokio-io-asyncbufreadext-bufreader
  - external/super
  - external/std-io-write
  - external/tempfile-tempdir
  - external/uuid-uuid
  - external/futures-future-join-all
  - external/std-sync-arc
  member_of:
  - packages/crustly
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