---
type: Rust Function
title: maybe_tool_call_json
resource: src/llm/provider/tool_call_recovery.rs#L25-L28
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/llama_cpp/run_stream
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
---

# Signature

`pub fn maybe_tool_call_json(text: &str) -> bool`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)