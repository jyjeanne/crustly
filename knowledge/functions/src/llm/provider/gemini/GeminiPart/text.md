---
type: Rust Method
title: text
resource: src/llm/provider/gemini.rs#L765-L770
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
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

`fn text(text: String) -> Self`

# Called by

- [to_gemini_request](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [execute](../../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [embed](../../../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [embed](../../../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)