---
type: Rust Module
title: ask_user
resource: src/llm/tools/ask_user.rs#L1-L177
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-io-write
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [AskUserTool](../../../../classes/src/llm/tools/ask_user/AskUserTool.md)
- [AskUserInput](../../../../classes/src/llm/tools/ask_user/AskUserInput.md)
- [name](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/ask_user/AskUserTool/tool/execute.md)
- [test_validate_empty_question](../../../../functions/src/llm/tools/ask_user/test_validate_empty_question.md)
- [test_validate_valid_question](../../../../functions/src/llm/tools/ask_user/test_validate_valid_question.md)
- [test_validate_with_context](../../../../functions/src/llm/tools/ask_user/test_validate_with_context.md)
- [test_auto_approve_returns_placeholder](../../../../functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::io::Write`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)