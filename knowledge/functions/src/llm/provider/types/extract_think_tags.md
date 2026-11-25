---
type: Rust Function
title: extract_think_tags
resource: src/llm/provider/types.rs#L439-L472
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags_single_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags_multiple_blocks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags_no_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags_unclosed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/extract_think_tags_only_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_think_tags(text: &str) -> (String, String)`

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [from_openai_response](../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [extract_think_tags_single_block](../../../../../functions/src/llm/provider/types/extract_think_tags_single_block.md)
- [extract_think_tags_multiple_blocks](../../../../../functions/src/llm/provider/types/extract_think_tags_multiple_blocks.md)
- [extract_think_tags_no_tags](../../../../../functions/src/llm/provider/types/extract_think_tags_no_tags.md)
- [extract_think_tags_unclosed](../../../../../functions/src/llm/provider/types/extract_think_tags_unclosed.md)
- [extract_think_tags_only_block](../../../../../functions/src/llm/provider/types/extract_think_tags_only_block.md)