---
type: Rust Method
title: status
resource: src/memory/code_knowledge.rs#L291-L296
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/probe_executable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_search/WebSearchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/is_ready
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/spawn_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OllamaEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn status(&self) -> Status`

# Called by

- [complete](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [complete](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [complete](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [complete](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [probe_executable](../../../../../functions/src/llm/tools/powershell/probe_executable.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)
- [is_ready](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/is_ready.md)
- [status_line](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status_line.md)
- [start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/start.md)
- [spawn_start](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/spawn_start.md)
- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)
- [embed](../../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [embed](../../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)