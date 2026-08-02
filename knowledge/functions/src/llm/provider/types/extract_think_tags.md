---
type: Rust Function
title: extract_think_tags
resource: src/llm/provider/types.rs#L439-L472
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/service/drain_stream_to_response
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
  - functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
  - functions/src/llm/provider/types/extract_think_tags_single_block
  - functions/src/llm/provider/types/extract_think_tags_multiple_blocks
  - functions/src/llm/provider/types/extract_think_tags_no_tags
  - functions/src/llm/provider/types/extract_think_tags_unclosed
  - functions/src/llm/provider/types/extract_think_tags_only_block
---

# Signature

`pub fn extract_think_tags(text: &str) -> (String, String)`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [from_openai_response](../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)
- [extract_think_tags_single_block](../../../../../functions/src/llm/provider/types/extract_think_tags_single_block.md)
- [extract_think_tags_multiple_blocks](../../../../../functions/src/llm/provider/types/extract_think_tags_multiple_blocks.md)
- [extract_think_tags_no_tags](../../../../../functions/src/llm/provider/types/extract_think_tags_no_tags.md)
- [extract_think_tags_unclosed](../../../../../functions/src/llm/provider/types/extract_think_tags_unclosed.md)
- [extract_think_tags_only_block](../../../../../functions/src/llm/provider/types/extract_think_tags_only_block.md)