---
type: Rust Function
title: parse_tool_call_object
resource: src/llm/provider/ollama.rs#L885-L921
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/llm/provider/ollama/tool_call_from_content
---

# Signature

`fn parse_tool_call_object(text: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>`

# Calls

- [from_str](../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [tool_call_from_content](../../../../../functions/src/llm/provider/ollama/tool_call_from_content.md)