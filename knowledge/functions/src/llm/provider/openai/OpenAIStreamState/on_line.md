---
type: Rust Method
title: on_line
resource: src/llm/provider/openai.rs#L1015-L1117
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn on_line(&mut self, line: &str, events: &mut Vec<StreamEvent>)`

# Called by

- [on_bytes](../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)
- [finish](../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/finish.md)