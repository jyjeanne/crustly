---
type: Rust Function
title: schema_has_one_variant_per_offered_tool
resource: src/llm/provider/llama_cpp_grammar.rs#L173-L177
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/read_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema_has_one_variant_per_offered_tool()`

# Calls

- [tool_call_json_schema](../../../../../functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema.md)
- [read_tool](../../../../../functions/src/llm/provider/llama_cpp_grammar/read_tool.md)