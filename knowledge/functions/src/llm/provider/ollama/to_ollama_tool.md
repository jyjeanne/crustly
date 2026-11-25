---
type: Rust Function
title: to_ollama_tool
resource: src/llm/provider/ollama.rs#L862-L879
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/to_ollama_tool_converts_valid_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_tool_falls_back_on_invalid_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_ollama_tool(tool: &Tool) -> ToolInfo`

# Called by

- [to_ollama_tool_converts_valid_schema](../../../../../functions/src/llm/provider/ollama/to_ollama_tool_converts_valid_schema.md)
- [to_ollama_tool_falls_back_on_invalid_schema](../../../../../functions/src/llm/provider/ollama/to_ollama_tool_falls_back_on_invalid_schema.md)