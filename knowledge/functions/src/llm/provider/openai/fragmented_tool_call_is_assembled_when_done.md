---
type: Rust Function
title: fragmented_tool_call_is_assembled_when_done
resource: src/llm/provider/openai.rs#L1499-L1549
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/sse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn fragmented_tool_call_is_assembled_when_done()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [sse](../../../../../functions/src/llm/provider/openai/sse.md)
- [on_bytes](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)
- [finish](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/finish.md)