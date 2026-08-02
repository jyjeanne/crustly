---
type: Rust Function
title: parse_tool_call_object
resource: src/llm/provider/tool_call_recovery.rs#L232-L268
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/llm/provider/tool_call_recovery/tool_call_from_content
---

# Signature

`fn parse_tool_call_object(text: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>`

# Calls

- [from_str](../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [tool_call_from_content](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)