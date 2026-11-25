---
type: Rust Function
title: parse_method
resource: src/llm/tools/http.rs#L54-L68
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/http/HttpClientTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_method(method_str: &str) -> Result<Method>`

# Called by

- [validate_input](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)