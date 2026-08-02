---
type: Rust Method
title: stream
resource: src/llm/provider/ollama.rs#L544-L714
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json
  - functions/src/llm/provider/ollama/collect_tool_calls
  - functions/src/llm/provider/ollama/perf_metrics_from_final_data
  - functions/src/llm/provider/tool_call_recovery/tool_call_from_content
  - functions/src/llm/provider/ollama/stop_reason_for
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_ollama_request](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [maybe_tool_call_json](../../../../../../../functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json.md)
- [collect_tool_calls](../../../../../../../functions/src/llm/provider/ollama/collect_tool_calls.md)
- [perf_metrics_from_final_data](../../../../../../../functions/src/llm/provider/ollama/perf_metrics_from_final_data.md)
- [tool_call_from_content](../../../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)
- [stop_reason_for](../../../../../../../functions/src/llm/provider/ollama/stop_reason_for.md)