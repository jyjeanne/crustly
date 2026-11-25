---
type: Rust Method
title: new
resource: src/llm/provider/ollama.rs#L167-L189
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_invalid_host_falls_back_to_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/ollama_download/build_ollama_provider
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(host: impl Into<String>) -> Self`

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [default_local](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [test_invalid_host_falls_back_to_default](../../../../../../functions/src/llm/provider/ollama/test_invalid_host_falls_back_to_default.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [build_ollama_provider](../../../../../../functions/src/tui/ollama_download/build_ollama_provider.md)