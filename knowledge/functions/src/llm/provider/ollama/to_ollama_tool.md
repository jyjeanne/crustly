---
type: Rust Function
title: to_ollama_tool
resource: src/llm/provider/ollama.rs#L804-L821
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/to_ollama_tool_converts_valid_schema
  - functions/src/llm/provider/ollama/to_ollama_tool_falls_back_on_invalid_schema
---

# Signature

`fn to_ollama_tool(tool: &Tool) -> ToolInfo`

# Called by

- [to_ollama_tool_converts_valid_schema](../../../../../functions/src/llm/provider/ollama/to_ollama_tool_converts_valid_schema.md)
- [to_ollama_tool_falls_back_on_invalid_schema](../../../../../functions/src/llm/provider/ollama/to_ollama_tool_falls_back_on_invalid_schema.md)