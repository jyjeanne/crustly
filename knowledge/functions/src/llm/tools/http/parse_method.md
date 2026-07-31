---
type: Rust Function
title: parse_method
resource: src/llm/tools/http.rs#L54-L68
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/http/HttpClientTool/tool/validate_input
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
---

# Signature

`fn parse_method(method_str: &str) -> Result<Method>`

# Called by

- [validate_input](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)