---
type: Rust Function
title: to_ollama_format
resource: src/llm/provider/ollama.rs#L883-L890
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/to_ollama_format_json_object_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_format_structured_schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_ollama_format(value: &serde_json::Value) -> Option<FormatType>`

# Called by

- [to_ollama_format_json_object_marker](../../../../../functions/src/llm/provider/ollama/to_ollama_format_json_object_marker.md)
- [to_ollama_format_structured_schema](../../../../../functions/src/llm/provider/ollama/to_ollama_format_structured_schema.md)