---
type: Rust Module
title: ask_user
resource: src/llm/tools/ask_user.rs#L1-L177
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-error-result-toolerror
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
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
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