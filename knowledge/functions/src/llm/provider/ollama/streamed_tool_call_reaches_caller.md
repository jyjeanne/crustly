---
type: Rust Function
title: streamed_tool_call_reaches_caller
resource: src/llm/provider/ollama.rs#L1551-L1593
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/types/LLMRequest/with_tools
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn streamed_tool_call_reaches_caller()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_tools](../../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)