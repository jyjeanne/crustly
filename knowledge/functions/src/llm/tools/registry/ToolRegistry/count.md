---
type: Rust Method
title: count
resource: src/llm/tools/registry.rs#L281-L283
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/build_tool_registry_registers_every_built_in_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_registry_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_register_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn count(&self) -> usize`

# Called by

- [build_tool_registry_registers_every_built_in_tool](../../../../../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [test_registry_creation](../../../../../../functions/src/llm/tools/registry/test_registry_creation.md)
- [test_register_tool](../../../../../../functions/src/llm/tools/registry/test_register_tool.md)
- [register_mcp_server_with_nonexistent_command_fails_gracefully](../../../../../../functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully.md)