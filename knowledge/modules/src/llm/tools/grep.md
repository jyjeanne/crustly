---
type: Rust Module
title: grep
resource: src/llm/tools/grep.rs#L1-L486
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-path-pathbuf
  - external/tokio-fs
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [GrepTool](../../../../classes/src/llm/tools/grep/GrepTool.md)
- [GrepInput](../../../../classes/src/llm/tools/grep/GrepInput.md)
- [default_true](../../../../functions/src/llm/tools/grep/default_true.md)
- [name](../../../../functions/src/llm/tools/grep/GrepTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/grep/GrepTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/grep/GrepTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/grep/GrepTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/grep/GrepTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/grep/GrepTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [search_file](../../../../functions/src/llm/tools/grep/GrepTool/search_file.md)
- [collect_searchable_files](../../../../functions/src/llm/tools/grep/collect_searchable_files.md)
- [test_grep_accepts_glob_alias_for_file_pattern](../../../../functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern.md)
- [test_pattern_is_regex_by_default](../../../../functions/src/llm/tools/grep/test_pattern_is_regex_by_default.md)
- [test_regex_false_still_searches_literally](../../../../functions/src/llm/tools/grep/test_regex_false_still_searches_literally.md)
- [test_search_respects_gitignore](../../../../functions/src/llm/tools/grep/test_search_respects_gitignore.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)