---
type: Rust Module
title: write
resource: src/llm/tools/write.rs#L1-L478
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-validate-path-safety-result-toolerror
  - external/super-file-read-cache-filefingerprint-readgate
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

- [WriteTool](../../../../classes/src/llm/tools/write/WriteTool.md)
- [WriteInput](../../../../classes/src/llm/tools/write/WriteInput.md)
- [name](../../../../functions/src/llm/tools/write/WriteTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/write/WriteTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/write/WriteTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/write/WriteTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/write/WriteTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/write/WriteTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [test_write_file](../../../../functions/src/llm/tools/write/test_write_file.md)
- [test_write_file_with_create_dirs](../../../../functions/src/llm/tools/write/test_write_file_with_create_dirs.md)
- [test_write_file_missing_parent_dir](../../../../functions/src/llm/tools/write/test_write_file_missing_parent_dir.md)
- [test_write_file_accepts_file_path_alias](../../../../functions/src/llm/tools/write/test_write_file_accepts_file_path_alias.md)
- [test_write_tool_schema](../../../../functions/src/llm/tools/write/test_write_tool_schema.md)
- [test_overwrite_existing_file](../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)
- [test_creating_a_new_file_needs_no_prior_read](../../../../functions/src/llm/tools/write/test_creating_a_new_file_needs_no_prior_read.md)
- [test_write_then_overwrite_does_not_require_a_re_read](../../../../functions/src/llm/tools/write/test_write_then_overwrite_does_not_require_a_re_read.md)

# Imports

- `super::error::{validate_path_safety, Result, ToolError}`
- `super::file_read_cache::{FileFingerprint, ReadGate}`
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