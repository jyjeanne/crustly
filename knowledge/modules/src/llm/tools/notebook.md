---
type: Rust Module
title: notebook
resource: src/llm/tools/notebook.rs#L1-L409
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-pathbuf
  - external/tokio-fs
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [NotebookEditTool](../../../../classes/src/llm/tools/notebook/NotebookEditTool.md)
- [NotebookOperation](../../../../classes/src/llm/tools/notebook/NotebookOperation.md)
- [NotebookInput](../../../../classes/src/llm/tools/notebook/NotebookInput.md)
- [default_true](../../../../functions/src/llm/tools/notebook/default_true.md)
- [Notebook](../../../../classes/src/llm/tools/notebook/Notebook.md)
- [Cell](../../../../classes/src/llm/tools/notebook/Cell.md)
- [name](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [minimal_notebook_json](../../../../functions/src/llm/tools/notebook/minimal_notebook_json.md)
- [test_add_cell_within_working_directory_succeeds](../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)
- [test_tool_schema](../../../../functions/src/llm/tools/notebook/test_tool_schema.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::PathBuf`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)