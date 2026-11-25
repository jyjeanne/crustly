---
type: Rust Method
title: complete
resource: src/llm/provider/gemini.rs#L548-L597
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/generate_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_gemini_request](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [generate_url](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/generate_url.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/headers.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/handle_error.md)
- [from_gemini_response](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response.md)