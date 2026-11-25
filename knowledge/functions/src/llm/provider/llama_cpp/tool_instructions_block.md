---
type: Rust Function
title: tool_instructions_block
resource: src/llm/provider/llama_cpp.rs#L1391-L1406
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/tool_instructions_block_names_every_offered_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_instructions_block(tools: &[Tool]) -> String`

# Called by

- [tool_instructions_block_names_every_offered_tool](../../../../../functions/src/llm/provider/llama_cpp/tool_instructions_block_names_every_offered_tool.md)