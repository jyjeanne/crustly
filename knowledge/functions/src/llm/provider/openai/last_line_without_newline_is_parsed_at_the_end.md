---
type: Rust Function
title: last_line_without_newline_is_parsed_at_the_end
resource: src/llm/provider/openai.rs#L1552-L1561
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
  - target: functions/src/llm/provider/openai/OpenAIStreamState/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/text_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn last_line_without_newline_is_parsed_at_the_end()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [finish](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/finish.md)
- [text_chunk](../../../../../functions/src/llm/provider/openai/text_chunk.md)
- [on_bytes](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)