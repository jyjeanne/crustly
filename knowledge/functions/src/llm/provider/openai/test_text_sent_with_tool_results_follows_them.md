---
type: Rust Function
title: test_text_sent_with_tool_results_follows_them
resource: src/llm/provider/openai.rs#L1373-L1403
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_text_sent_with_tool_results_follows_them()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [to_openai_request](../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)