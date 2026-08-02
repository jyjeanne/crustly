---
type: Rust Module
title: glob
resource: src/llm/tools/glob.rs#L1-L294
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-pathbuf
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [GlobTool](../../../../classes/src/llm/tools/glob/GlobTool.md)
- [GlobInput](../../../../classes/src/llm/tools/glob/GlobInput.md)
- [name](../../../../functions/src/llm/tools/glob/GlobTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/glob/GlobTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/glob/GlobTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/glob/GlobTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/glob/GlobTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/glob/GlobTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [context](../../../../functions/src/llm/tools/glob/context.md)
- [test_glob_matches_recursive_pattern](../../../../functions/src/llm/tools/glob/test_glob_matches_recursive_pattern.md)
- [test_glob_respects_gitignore](../../../../functions/src/llm/tools/glob/test_glob_respects_gitignore.md)
- [test_glob_no_matches](../../../../functions/src/llm/tools/glob/test_glob_no_matches.md)
- [test_glob_respects_limit](../../../../functions/src/llm/tools/glob/test_glob_respects_limit.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::PathBuf`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)