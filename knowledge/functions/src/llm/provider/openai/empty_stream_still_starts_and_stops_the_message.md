---
type: Rust Function
title: empty_stream_still_starts_and_stops_the_message
resource: src/llm/provider/openai.rs#L1564-L1570
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
  - target: functions/src/llm/provider/openai/OpenAIStreamState/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn empty_stream_still_starts_and_stops_the_message()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [finish](../../../../../functions/src/llm/provider/openai/OpenAIStreamState/finish.md)