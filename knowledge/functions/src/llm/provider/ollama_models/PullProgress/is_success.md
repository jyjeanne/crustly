---
type: Rust Method
title: is_success
resource: src/llm/provider/ollama_models.rs#L42-L44
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
  - functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
  - functions/src/llm/provider/gemini/GeminiProvider/provider/complete
  - functions/src/llm/provider/gemini/GeminiProvider/provider/stream
  - functions/src/llm/provider/openai/OpenAIProvider/provider/complete
  - functions/src/llm/provider/openai/OpenAIProvider/provider/stream
  - functions/src/llm/provider/qwen/QwenProvider/provider/complete
  - functions/src/llm/provider/qwen/QwenProvider/provider/stream
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
  - functions/src/llm/tools/web_search/WebSearchTool/tool/execute
---

# Signature

`pub fn is_success(&self) -> bool`

# Called by

- [complete](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [complete](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [execute](../../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)