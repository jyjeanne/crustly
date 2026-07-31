---
type: Rust Function
title: fenced_json_blocks
resource: src/llm/provider/ollama.rs#L862-L880
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/tool_call_from_content
---

# Signature

`fn fenced_json_blocks(content: &str) -> Vec<&str>`

# Called by

- [tool_call_from_content](../../../../../functions/src/llm/provider/ollama/tool_call_from_content.md)