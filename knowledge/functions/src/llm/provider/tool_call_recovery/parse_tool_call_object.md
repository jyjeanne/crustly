---
type: Rust Function
title: parse_tool_call_object
resource: src/llm/provider/tool_call_recovery.rs#L232-L268
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_from_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_tool_call_object(text: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>`

# Called by

- [tool_call_from_content](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)