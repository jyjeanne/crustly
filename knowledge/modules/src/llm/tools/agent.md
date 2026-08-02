---
type: Rust Module
title: agent
resource: src/llm/tools/agent.rs#L1-L258
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/chrono-utc
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/uuid-uuid
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [AgentTool](../../../../classes/src/llm/tools/agent/AgentTool.md)
- [AgentInput](../../../../classes/src/llm/tools/agent/AgentInput.md)
- [AgentManifest](../../../../classes/src/llm/tools/agent/AgentManifest.md)
- [name](../../../../functions/src/llm/tools/agent/AgentTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/agent/AgentTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/agent/AgentTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/agent/AgentTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/agent/AgentTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/agent/AgentTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [slugify](../../../../functions/src/llm/tools/agent/slugify.md)
- [test_slugify](../../../../functions/src/llm/tools/agent/test_slugify.md)
- [test_validate_empty_description](../../../../functions/src/llm/tools/agent/test_validate_empty_description.md)
- [test_validate_empty_prompt](../../../../functions/src/llm/tools/agent/test_validate_empty_prompt.md)
- [test_validate_valid_input](../../../../functions/src/llm/tools/agent/test_validate_valid_input.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `chrono::Utc`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `uuid::Uuid`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)