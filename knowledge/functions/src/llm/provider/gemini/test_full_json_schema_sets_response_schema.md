---
type: Rust Function
title: test_full_json_schema_sets_response_schema
resource: src/llm/provider/gemini.rs#L1132-L1144
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_response_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_full_json_schema_sets_response_schema()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_response_format](../../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [to_gemini_request](../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)