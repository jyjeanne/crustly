---
type: Rust Function
title: drain_stream_to_response
resource: src/llm/agent/service.rs#L406-L532
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/llm/agent/service/apply_streamed_tool_input
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/agent/service/route_text_delta
  - functions/src/llm/provider/types/extract_think_tags
  called_by:
  - functions/src/llm/agent/service/AgentService/call_provider_streaming
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/agent/service/drain_stream_to_response_carries_perf_metrics_through
  - functions/src/llm/agent/service/drain_stream_assembles_anthropic_tool_input_from_json_deltas
---

# Signature

`async fn drain_stream_to_response( stream: ProviderStream, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [apply_streamed_tool_input](../../../../../functions/src/llm/agent/service/apply_streamed_tool_input.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [route_text_delta](../../../../../functions/src/llm/agent/service/route_text_delta.md)
- [extract_think_tags](../../../../../functions/src/llm/provider/types/extract_think_tags.md)

# Called by

- [call_provider_streaming](../../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [drain_stream_to_response_carries_perf_metrics_through](../../../../../functions/src/llm/agent/service/drain_stream_to_response_carries_perf_metrics_through.md)
- [drain_stream_assembles_anthropic_tool_input_from_json_deltas](../../../../../functions/src/llm/agent/service/drain_stream_assembles_anthropic_tool_input_from_json_deltas.md)