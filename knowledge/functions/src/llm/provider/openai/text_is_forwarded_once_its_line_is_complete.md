---
type: Rust Function
title: text_is_forwarded_once_its_line_is_complete
resource: src/llm/provider/openai.rs#L1477-L1496
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/sse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn text_is_forwarded_once_its_line_is_complete()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [sse](../../../../../functions/src/llm/provider/openai/sse.md)
- [text_chunk](../../../../../functions/src/llm/provider/openai/text_chunk.md)
- [on_bytes](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)